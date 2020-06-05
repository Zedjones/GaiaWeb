mod graphql;
mod models;
mod routes;
mod schema;

// Still need this because Diesel is a bit outdated until 2.0
#[macro_use]
extern crate diesel;

use env_logger::Env;
use lapin::ExchangeKind;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use log::{error, info};
use std::time::Duration;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use routes::get_routes;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
type WarpAddress = ([u8; 4], u16);

#[cfg(debug_assertions)]
const ADDR: &'static WarpAddress = &([127, 0, 0, 1], 8080);
#[cfg(not(debug_assertions))]
const ADDR: &'static WarpAddress = &([0, 0, 0, 0], 8080);

const CONN_TIMEOUT: Duration = Duration::from_secs(10);

async fn connect_timeout() -> Option<Connection> {
    let addr = std::env::var("RABBITMQ_ADDR").unwrap_or("127.0.0.1".to_string());
    let uri = format!("amqp://{}:5672/%2f", addr);
    let start = std::time::Instant::now();
    info!("Attempting to connect to RabbitMQ at address {}", uri);
    info!("Timeout is: {} seconds", CONN_TIMEOUT.as_secs());
    loop {
        if let Ok(conn) = Connection::connect(&uri, ConnectionProperties::default()).await {
            info!("Successfully connected to RabbitMQ server");
            break Some(conn);
        } else if start.elapsed() > CONN_TIMEOUT {
            break None;
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}

#[tokio::main]
async fn main() {
    env_logger::from_env(Env::default().default_filter_or("gaia=info,warp=info")).init();

    let rabbit_conn = match connect_timeout().await {
        Some(conn) => conn,
        None => {
            error!("Could not connect to RabbitMQ server");
            info!("Exiting...");
            std::process::exit(1);
        }
    };
    let send_chan = rabbit_conn.create_channel().await.unwrap();
    let _queue = send_chan.queue_declare(
        "gaia_input",
        QueueDeclareOptions::default(),
        FieldTable::default(),
    );
    let send_clone = send_chan.clone();

    let _exchange = send_chan.exchange_declare(
        "computation_updates",
        ExchangeKind::Fanout,
        ExchangeDeclareOptions::default(),
        FieldTable::default(),
    );

    let db_url = std::env::var("DATABASE_URL").unwrap_or("gaia.db".to_string());
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = r2d2::Pool::builder().build(manager).unwrap_or_else(|_| {
        error!("Could not build DB pool.");
        std::process::exit(1);
    });

    diesel_migrations::run_pending_migrations(&pool.get().unwrap()).unwrap();

    let routes = get_routes(pool, send_clone);
    warp::serve(routes).run(*ADDR).await;
}
