use actix_web::{web, App, HttpServer, Responder, middleware::Logger};
use actix_web::{HttpRequest, get, post, HttpResponse, Error};
use actix_multipart::Multipart;
use serde::Deserialize;
use log::info;
use env_logger::Env;
use lapin::{Connection, ConnectionProperties, options::*, types::FieldTable, Channel,
            BasicProperties, CloseOnDrop};
use futures::{StreamExt, TryStreamExt};
use actix_web::web::Bytes;
use std::time::Duration;

#[cfg(debug_assertions)]
    const ADDR: &'static str = "127.0.0.1:8000";
#[cfg(not(debug_assertions))]
    const ADDR: &'static str = "0.0.0.0:8000";

const CONN_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Deserialize)]
struct Info {
    name: String
}

#[post("/upload")]
async fn save_file(mut payload: Multipart, send_chan: web::Data<Channel>) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let mut my_vec: Vec<Bytes> = Vec::new();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            my_vec.push(data);
        }
        let all_bytes = my_vec.concat();
        send_chan.basic_publish(
            "",
            "jobs_input",
            BasicPublishOptions::default(),
            all_bytes,
            BasicProperties::default()
        ).await.unwrap();
    }
    Ok(HttpResponse::Ok().into())
}

#[get("/hello")]
async fn other_hello(name: web::Query<Info>) -> impl Responder {
    format!("Hello {}!", name.name)
}

#[get("/{name}{tail:.*}")]
async fn hello(req: HttpRequest, send_chan: web::Data<Channel>) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    send_chan.basic_publish(
        "",
        "jobs_input",
        BasicPublishOptions::default(),
        name.as_bytes().to_vec(),
        BasicProperties::default()
    ).await.unwrap();
    format!("Hello {}!", name)
}

async fn connect_timeout() -> Option<CloseOnDrop<Connection>> {
    let addr = std::env::var("RABBITMQ_ADDR").unwrap_or("127.0.0.1".to_string());
    let uri = format!("amqp://{}:5672/%2f", addr);
    let start = std::time::Instant::now();
    info!("Attempting to connect to RabbitMQ at address {}", uri);
    info!("Timeout is: {} seconds", CONN_TIMEOUT.as_secs());
    loop {
        if let Ok(conn) = Connection::connect(&uri, ConnectionProperties::default()).await {
            break Some(conn)
        }
        else if start.elapsed() > CONN_TIMEOUT {
            break None
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let conn = connect_timeout().await.unwrap();
    let send_chan = conn.create_channel().await.unwrap();
    let _queue = send_chan
        .queue_declare(
            "jobs_input",
            QueueDeclareOptions::default(),
            FieldTable::default()
        );
    let send_clone = send_chan.clone();

    std::fs::create_dir_all("./tmp").unwrap();
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