
use actix_multipart::Multipart;
use actix_web::{web, post, Responder, get, HttpRequest, HttpResponse, Error};
use lapin::{BasicProperties, Channel, options::*};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use web::{Bytes, Query, Data};

#[derive(Deserialize)]
struct Info {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Settings {
    data_id: Option<i32>,
    db_scan: bool,
    epsilon: i32,
    cluster_size: i32
}

#[post("/upload")]
async fn save_file(mut payload: Multipart, mut settings: Query<Settings>, 
                   send_chan: Data<Channel>) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let mut my_vec: Vec<Bytes> = Vec::new();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            my_vec.push(data);
        }
        // Currently not using this until we write to db
        let _all_bytes = my_vec.concat();
        // Random placeholder until we implement db
        settings.0.data_id = Some(12345);
        send_chan.basic_publish(
            "",
            "gaia_input",
            BasicPublishOptions::default(),
            serde_json::to_vec(&settings.0).unwrap(),
            BasicProperties::default()
        ).await.unwrap();
    }
    Ok(HttpResponse::Ok().into())
}

#[get("/hello")]
async fn other_hello(name: Query<Info>) -> impl Responder {
    format!("Hello {}!", name.name)
}

#[get("/{name}{tail:.*}")]
async fn hello(req: HttpRequest, send_chan: web::Data<Channel>) -> impl Responder {
    let name = req.match_info().get("name").unwrap();
    send_chan.basic_publish(
        "",
        "gaia_input",
        BasicPublishOptions::default(),
        name.as_bytes().to_vec(),
        BasicProperties::default()
    ).await.unwrap();
    format!("Hello {}!", name)
}