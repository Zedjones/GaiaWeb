
use actix_multipart::Multipart;
use actix_web::{web, post, Responder, get, HttpRequest, HttpResponse, Error};
use lapin::{BasicProperties, Channel, options::*};
use futures::{StreamExt, TryStreamExt};
use serde::Deserialize;
use web::Bytes;

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