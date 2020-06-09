use bytes::buf::Buf;
use diesel::prelude::*;
use futures::{Future, FutureExt, StreamExt, TryStreamExt};
use juniper::GraphQLInputObject;
use juniper_subscriptions::Coordinator;
use juniper_warp::subscriptions::graphql_subscriptions;
use lapin::{options::*, BasicProperties, Channel};
use log::info;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;
use warp::filters::multipart::FormData;
use warp::{Filter, Rejection, Reply};

use super::DbPool;
use crate::graphql::schema::{schema, Context};

fn default_db_scan() -> bool {
    false
}

fn default_epsilon() -> i32 {
    5
}

fn default_cluster_size() -> i32 {
    200
}

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject)]
pub(crate) struct PutSettings {
    pub email: String,
    pub title: String,
    #[serde(default = "default_db_scan")]
    #[graphql(default = "false")]
    pub db_scan: bool,
    #[serde(default = "default_epsilon")]
    #[graphql(default = "5")]
    pub epsilon: i32,
    #[serde(default = "default_cluster_size")]
    #[graphql(default = "200")]
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

async fn create_computation(
    mut payload: FormData,
    settings: PutSettings,
    send_chan: Channel,
    db_pool: DbPool,
) -> Result<impl Reply, Rejection> {
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
    Ok(warp::reply())
}

async fn get_computations(user_email: String, db_pool: DbPool) -> Result<impl Reply, Rejection> {
    use super::schema::computations::dsl::*;

    let db = db_pool.get().unwrap();

    let user_computations = computations
        .filter(email.eq(&user_email))
        .load::<super::models::Computation>(&db)
        .expect(&format!(
            "Error loading computations for user {}",
            user_email
        ));

    Ok(warp::reply::json(&user_computations))
}

fn with_pool(
    pool: DbPool,
) -> impl Filter<Extract = (DbPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn with_channel(
    chan: Channel,
) -> impl Filter<Extract = (Channel,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || chan.clone())
}

fn with_context(
    pool: DbPool,
    chan: Channel,
) -> impl Filter<Extract = (Context,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Context {
        pool: pool.clone(),
        channel: chan.clone(),
    })
}

pub fn get_routes(
    pool: DbPool,
    send_chan: Channel,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
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

    let graphql_filter = juniper_warp::make_graphql_filter(
        schema(),
        with_context(pool.clone(), send_chan.clone()).boxed(),
    );
    let graphql = warp::path("graphql").and(graphql_filter);
    let graphiql = warp::path("graphiql").and(juniper_warp::graphiql_filter("/graphql", None));
    let graphql_get = warp::get().and(graphql.clone().or(graphiql.clone()));
    let graphql_post = warp::post().and(graphql.or(graphiql));

    let coordinator = Arc::new(juniper_subscriptions::Coordinator::new(schema()));

    let subscriptions = warp::path("subscriptions")
        .and(warp::ws())
        .and(with_context(pool.clone(), send_chan.clone()))
        .and(warp::any().map(move || Arc::clone(&coordinator)))
        .map(
            |ws: warp::ws::Ws,
             ctx: Context,
             coordinator: Arc<Coordinator<'static, _, _, _, _, _>>| {
                ws.on_upgrade(|websocket| -> Pin<Box<dyn Future<Output = ()> + Send>> {
                    graphql_subscriptions(websocket, coordinator, ctx)
                        .map(|r| {
                            if let Err(e) = r {
                                log::error!("Websocket error: {}", e);
                            }
                        })
                        .boxed()
                })
            },
        )
        .map(|reply| warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws"));

    let log = warp::log("gaia_web");
    subscriptions
        .or(graphql_get
            .or(graphql_post.or(get_user_computations.or(put_computation.or(react_files)))))
        .with(log)
}
