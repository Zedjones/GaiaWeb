
use actix_multipart::Multipart;
use actix_web::{web, put, get, HttpResponse, Error, Responder};
use lapin::{BasicProperties, Channel, options::*};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use web::{Bytes, Query, Data};
use log::info;
use diesel::prelude::*;

use super::DbPool;

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
struct PutSettings {
    data_id: Option<i32>,
    filename: Option<String>,
    email: String,
    #[serde(default = "default_db_scan")]
    db_scan: bool,
    #[serde(default = "default_epsilon")]
    epsilon: i32,
    #[serde(default = "default_cluster_size")]
    cluster_size: i32
}

#[derive(Deserialize)]
struct GetSettings {
    email: String
}

#[derive(Serialize, Debug)]
struct UserComputation {
    computation: Vec<super::models::Computation>,
    clusters: Vec<i32>
}

#[put("/computation")]
async fn create_computation(mut payload: Multipart, mut settings: Query<PutSettings>, 
                            send_chan: Data<Channel>, db_pool: Data<DbPool>) -> Result<HttpResponse, Error> {
    use super::models::NewComputation;
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let mut my_vec: Vec<Bytes> = Vec::new();
        let filename = field.content_disposition().unwrap().get_filename().unwrap().to_string();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            my_vec.push(data);
        }
        let csv_file = my_vec.concat();
        let new_comp = NewComputation {
            email: settings.0.email.clone(),
            csv_file
        };
        let comp = new_comp.insert_computation(&db_pool);
        info!("Created computation with id: {}", comp.id);
        settings.0.data_id = Some(comp.id);
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

#[get("/computation")]
async fn get_computations(settings: Query<GetSettings>, db_pool: Data<DbPool>) -> impl Responder {
    use super::schema::computations::dsl::*;

    let db = db_pool.get().unwrap();

    let user_computations = computations
        .filter(email.eq(&settings.0.email))
        .load::<super::models::Computation>(&db)
        .expect(&format!("Error loading computations for user {}", &settings.0.email));

    HttpResponse::Ok().json(user_computations)
}