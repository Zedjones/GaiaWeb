mod routes;

use actix_web::{App, HttpServer, middleware::Logger};
use log::{info, error};
use env_logger::Env;
use lapin::{Connection, ConnectionProperties, options::*, types::FieldTable, CloseOnDrop};
use std::time::Duration;

use diesel::prelude::*;
use diesel::connection::Connection as _;

use routes::{other_hello, hello, save_file};

#[cfg(debug_assertions)]
    const ADDR: &'static str = "127.0.0.1:8000";
#[cfg(not(debug_assertions))]
    const ADDR: &'static str = "0.0.0.0:8000";

const CONN_TIMEOUT: Duration = Duration::from_secs(10);

async fn connect_timeout() -> Option<CloseOnDrop<Connection>> {
    let addr = std::env::var("RABBITMQ_ADDR").unwrap_or("127.0.0.1".to_string());
    let uri = format!("amqp://{}:5672/%2f", addr);
    let start = std::time::Instant::now();
    info!("Attempting to connect to RabbitMQ at address {}", uri);
    info!("Timeout is: {} seconds", CONN_TIMEOUT.as_secs());
    loop {
        if let Ok(conn) = Connection::connect(&uri, ConnectionProperties::default()).await {
            info!("Successfully connected to RabbitMQ server");
            break Some(conn)
        }
        else if start.elapsed() > CONN_TIMEOUT {
            break None
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn establish_db_connection() -> SqliteConnection {
    let db_url = std::env::var("DATABASE_URL").unwrap_or("gaia.db".to_string());
    SqliteConnection::establish(&db_url).unwrap_or_else(|_| {
        error!("Could not connect to DB at {}", db_url);
        std::process::exit(1);
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("gaia=info,actix=info")).init();

    let rabbit_conn = match connect_timeout().await {
        Some(conn) => conn,
        None => {
            error!("Could not connect to RabbitMQ server");
            info!("Exiting...");
            std::process::exit(1);
        }
    };
    let send_chan = rabbit_conn.create_channel().await.unwrap();
    let _queue = send_chan
        .queue_declare(
            "gaia_input",
            QueueDeclareOptions::default(),
            FieldTable::default()
        );
    let send_clone = send_chan.clone();

    let db_conn = establish_db_connection();
    diesel_migrations::run_pending_migrations(&db_conn).unwrap();

    HttpServer::new(move || {
        App::new()
            .service(other_hello)
            .service(hello)
            .service(save_file)
            .data(send_clone.clone())
            .wrap(Logger::default())
    })
    .bind(ADDR)?
    .run()
    .await
}