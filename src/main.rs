use actix_web::{web, App, HttpServer, Responder, middleware::Logger};
use actix_web::{HttpRequest, get, post, HttpResponse, Error};
use actix_multipart::Multipart;
use serde::Deserialize;
use env_logger::Env;
use lapin::{Connection, ConnectionProperties, options::*, types::FieldTable, Channel,
            BasicProperties};
use std::io::Write;
use futures::{StreamExt, TryStreamExt};

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
async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", filename);
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap();
        }
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