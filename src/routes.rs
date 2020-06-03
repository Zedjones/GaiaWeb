
use lapin::{BasicProperties, Channel, options::*};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use log::info;
use diesel::prelude::*;
use warp::filters::multipart::FormData;
use warp::{Rejection, Reply, Filter};
use bytes::buf::Buf;

use super::DbPool;
use crate::graphql::schema::{Context, schema};

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
    email: String,
    title: String,
    #[serde(default = "default_db_scan")]
    db_scan: bool,
    #[serde(default = "default_epsilon")]
    epsilon: i32,
    #[serde(default = "default_cluster_size")]
    cluster_size: i32
}

#[derive(Serialize, Debug)]
struct UserComputation {
    computation: Vec<super::models::Computation>,
    clusters: Vec<i32>
}

async fn create_computation(mut payload: FormData, mut settings: PutSettings, 
                            send_chan: Channel, db_pool: DbPool) -> Result<impl Reply, Rejection> {
    use super::models::NewComputation;
    // iterate over multipart stream
    while let Ok(Some(part)) = payload.try_next().await {
        let mut csv_file: Vec<u8> = Vec::new();
        let mut field = part.stream();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            csv_file.append(&mut data.bytes().to_vec());
        }
        log::info!("Length of file: {}", csv_file.len());
        let new_comp = NewComputation {
            email: settings.email.clone(),
            title: settings.title.clone(),
            csv_file,
        };
        let comp = new_comp.insert_computation(&db_pool);
        info!("Created computation with id: {}", comp.id);
        settings.data_id = Some(comp.id);
        send_chan.basic_publish(
            "",
            "gaia_input",
            BasicPublishOptions::default(),
            serde_json::to_vec(&settings).unwrap(),
            BasicProperties::default()
        ).await.unwrap();
    }
    Ok(warp::reply())
}

async fn get_computations(user_email: String, db_pool: DbPool) -> Result<impl Reply, Rejection> {
    use super::schema::computations::dsl::*;

    let db = db_pool.get().unwrap();

    let user_computations = computations
        .filter(email.eq(&user_email))
        .load::<super::models::Computation>(&db)
        .expect(&format!("Error loading computations for user {}", user_email));

    Ok(warp::reply::json(&user_computations))
}

fn with_pool(pool: DbPool) -> impl Filter<Extract = (DbPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn with_context(pool: DbPool) -> impl Filter<Extract = (Context,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Context{ pool: pool.clone() })
}

fn with_channel(chan: Channel) -> impl Filter<Extract = (Channel,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || chan.clone())
}

pub fn get_routes(pool: DbPool, send_chan: Channel) -> 
        impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    let react_files = warp::get().and(warp::fs::dir("frontend/build/"));

    let put_computation = warp::path("computation")
        .and(warp::put())
        // Set max size to 1 GB
        .and(warp::multipart::form().max_length(1_000_000_000))
        .and(warp::query::query::<PutSettings>())
        .and(with_channel(send_chan.clone()))
        .and(with_pool(pool.clone()))
        .and_then(create_computation);

    let get_user_computations = warp::path!("computation" / String)
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and_then(get_computations);

    let graphql_filter = juniper_warp::make_graphql_filter(schema(), with_context(pool.clone()).boxed());
    let graphql = warp::path("graphql")
        .and(warp::get())
        .and(graphql_filter);

    let graphiql = warp::path("graphiql")
        .and(warp::get())
        .and(juniper_warp::graphiql_filter("/graphql", None));

    let log = warp::log("gaia_web");
    graphiql.or(graphql.or(get_user_computations.or(put_computation.or(react_files)))).with(log)
}