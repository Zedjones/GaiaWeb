use actix_files as fs;
use actix_multipart::Multipart;
use actix_web::{get, post, put, web, Error, HttpRequest, HttpResponse, Responder, Result};
use actix_web_actors::ws;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::InputObject;
use async_graphql_actix_web::{GQLRequest, GQLResponse, WSSubscription};
use diesel::prelude::*;
use futures::{StreamExt, TryStreamExt};
use lapin::{options::*, BasicProperties, Channel};
use log::info;
use serde::{Deserialize, Serialize};
use web::{Bytes, Data, Query};

use super::DbPool;
use crate::graphql::schema::Schema;

fn default_db_scan() -> bool {
    false
}

fn default_epsilon() -> i32 {
    5
}

fn default_cluster_size() -> i32 {
    200
}

#[InputObject]
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PutSettings {
    pub email: String,
    pub title: String,
    #[serde(default = "default_db_scan")]
    #[field(default = false)]
    pub db_scan: bool,
    #[serde(default = "default_epsilon")]
    #[field(default = 5)]
    pub epsilon: i32,
    #[serde(default = "default_cluster_size")]
    #[field(default = 200)]
    pub cluster_size: i32,
}

#[derive(Serialize)]
pub(crate) struct QueueMessage<'a> {
    #[serde(flatten)]
    pub settings: &'a PutSettings,
    pub data_id: i32,
}

#[derive(Serialize, Debug)]
struct UserComputation {
    computation: Vec<super::models::Computation>,
    clusters: Vec<i32>,
}

#[put("/computation")]
async fn create_computation(
    mut payload: Multipart,
    settings: Query<PutSettings>,
    send_chan: Data<Channel>,
    db_pool: Data<DbPool>,
) -> Result<HttpResponse, Error> {
    use super::models::NewComputation;
    while let Ok(Some(mut field)) = payload.try_next().await {
        let mut my_vec: Vec<Bytes> = Vec::new();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            my_vec.push(data);
        }
        let csv_file = my_vec.concat();
        log::info!("Length of file: {}", csv_file.len());
        let new_comp = NewComputation {
            email: settings.email.clone(),
            title: settings.title.clone(),
            csv_file,
        };
        let comp = new_comp.insert_computation(&db_pool);
        info!("Created computation with id: {}", comp.id);
        let queue_message = QueueMessage {
            data_id: comp.id,
            settings: &settings,
        };
        send_chan
            .basic_publish(
                "",
                "gaia_input",
                BasicPublishOptions::default(),
                serde_json::to_vec(&queue_message).unwrap(),
                BasicProperties::default(),
            )
            .await
            .unwrap();
        send_chan
            .basic_publish(
                "computation_updates",
                "",
                BasicPublishOptions::default(),
                serde_json::to_vec(&serde_json::json!({
                    "email": settings.email,
                    "id": queue_message.data_id,
                }))
                .unwrap(),
                BasicProperties::default(),
            )
            .await
            .unwrap();
    }
    Ok(HttpResponse::Ok().into())
}

#[get("/computation/{email}")]
async fn get_computations(user_email: web::Path<String>, db_pool: Data<DbPool>) -> impl Responder {
    use super::schema::computations::dsl::*;

    let db = db_pool.get().unwrap();

    let user_computations = computations
        .filter(email.eq(&user_email.to_string()))
        .load::<super::models::Computation>(&db)
        .expect(&format!(
            "Error loading computations for user {}",
            &user_email
        ));

    HttpResponse::Ok().json(user_computations)
}

pub(crate) async fn graphql_ws(
    schema: web::Data<Schema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    ws::start_with_protocols(WSSubscription::new(&schema), &["graphql-ws"], &req, payload)
}

#[post("/graphql")]
async fn graphql(schema: web::Data<Schema>, req: GQLRequest) -> GQLResponse {
    req.into_inner().execute(&schema).await.into()
}

#[get("/graphql")]
async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/subscriptions"),
        )))
}

#[get("/")]
async fn index() -> impl Responder {
    fs::NamedFile::open("./frontend/build/index.html")
}
