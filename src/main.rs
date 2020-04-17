use actix_web::{web, App, HttpServer, Responder, middleware::Logger};
use actix_web::{HttpRequest, get};
use serde::Deserialize;
use env_logger::Env;
use lapin::{Connection, ConnectionProperties, options::*, types::FieldTable, Channel,
            BasicProperties};

cfg_if::cfg_if! {
    if #[cfg(debug_assertions)] {
        const ADDR: &'static str = "127.0.0.1:8000";
    }
    else {
        const ADDR: &'static str = "0.0.0.0:8000";
    }
}

#[derive(Deserialize)]
struct Info {
    name: String
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
        "pokemon",
        BasicPublishOptions::default(),
        name.as_bytes().to_vec(),
        BasicProperties::default()
    ).await.unwrap();
    format!("Hello {}!", name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = "amqp://127.0.0.1:5672/%2f";
    let conn = Connection::connect(addr, ConnectionProperties::default()).await.unwrap();
    let send_chan = conn.create_channel().await.unwrap();
    let _queue = send_chan
        .queue_declare(
            "pokemon",
            QueueDeclareOptions::default(),
            FieldTable::default()
        );
    let send_clone = send_chan.clone();
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    HttpServer::new(move || {
        App::new()
            .service(other_hello)
            .service(hello)
            .data(send_clone.clone())
            .wrap(Logger::default())
    })
    .bind(ADDR)?
    .run()
    .await
}