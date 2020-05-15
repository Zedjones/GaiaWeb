
use actix_multipart::Multipart;
use actix_web::{web, post, HttpResponse, Error};
use lapin::{BasicProperties, Channel, options::*};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use web::{Bytes, Query, Data};
use log::info;

use super::DbPool;
use super::models::NewComputation;

fn default_db_scan() -> bool {
    false
}

fn default_epsilon() -> i32 {
    5
}

fn default_cluster_size() -> i32 {
    200
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    data_id: Option<Vec<u8>>,
    filename: Option<String>,
    email: String,
    #[serde(default = "default_db_scan")]
    db_scan: bool,
    #[serde(default = "default_epsilon")]
    epsilon: i32,
    #[serde(default = "default_cluster_size")]
    cluster_size: i32
}

#[post("/upload")]
async fn save_file(mut payload: Multipart, mut settings: Query<Settings>, 
                   send_chan: Data<Channel>, db_pool: Data<DbPool>) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let mut my_vec: Vec<Bytes> = Vec::new();
        let filename = field.content_disposition().unwrap().get_filename().unwrap().to_string();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            my_vec.push(data);
        }
        // Currently not using this until we write to db
        let csv_file = my_vec.concat();
        let new_comp = NewComputation {
            email: settings.0.email.clone(),
            csv_file
        };
        let comp = new_comp.insert_computation(&db_pool);
        info!("Created computation with id: {}", comp.id);
        // Random placeholder until we implement db
        settings.0.filename = Some(filename);
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