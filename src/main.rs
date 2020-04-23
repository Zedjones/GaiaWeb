use actix_web::{web, App, HttpServer, Responder, middleware::Logger};
use actix_web::{HttpRequest, get, post, HttpResponse, Error};
use actix_multipart::Multipart;
use serde::Deserialize;
use env_logger::Env;
use lapin::{Connection, ConnectionProperties, options::*, types::FieldTable, Channel,
            BasicProperties};
use std::io::Write;
use futures::{StreamExt, TryStreamExt};
use actix_web::web::Bytes;

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

#[post("/upload")]
async fn save_file(mut payload: Multipart, send_chan: web::Data<Channel>) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", filename);
        let mut my_vec: Vec<Bytes> = Vec::new();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            my_vec.push(data);
            // filesystem operations are blocking, we have to use threadpool
        }
        let all_bytes = my_vec.concat();
        send_chan.basic_publish(
            "",
            "pokemon",
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