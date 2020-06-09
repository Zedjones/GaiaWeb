use crate::models::Computation;
use crate::routes::{PutSettings, QueueMessage};
use crate::DbPool;
use diesel::prelude::*;
use futures::Stream;
use juniper::{FieldError, RootNode};
use lapin::{options::*, types::FieldTable, BasicProperties};
use log::info;
use serde::Deserialize;
use std::pin::Pin;

type Schema = RootNode<'static, Query, Mutation, Subscription>;

pub(crate) fn schema() -> Schema {
    Schema::new(Query, Mutation, Subscription)
}

#[derive(Clone)]
pub(crate) struct Context {
    pub pool: DbPool,
    pub channel: lapin::Channel,
}

impl juniper::Context for Context {}

pub(crate) struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    #[graphql(arguments(user_email(name = "email")))]
    fn get_computations(context: &Context, user_email: String) -> Vec<Computation> {
        use crate::schema::computations::dsl::*;

        let db = context.pool.get().unwrap();

        computations
            .filter(email.eq(&user_email))
            .load::<crate::models::Computation>(&db)
            .expect(&format!(
                "Error loading computations for user {}",
                user_email
            ))
    }
}

pub(crate) struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    fn add_computation(context: &Context, settings: PutSettings, csv_file: String) -> Computation {
        use crate::models::NewComputation;
        let new_comp = NewComputation {
            email: settings.email.clone(),
            title: settings.title.clone(),
            csv_file: base64::decode(csv_file).unwrap(),
        };
        let comp = new_comp.insert_computation(&context.pool);
        info!("Created computation with id: {}", comp.id);
        let queue_message = QueueMessage {
            data_id: comp.id,
            settings: &settings,
        };
        futures::executor::block_on(async {
            context
                .channel
                .basic_publish(
                    "",
                    "gaia_input",
                    BasicPublishOptions::default(),
                    serde_json::to_vec(&queue_message).unwrap(),
                    BasicProperties::default(),
                )
                .await
                .unwrap()
        });
        futures::executor::block_on(async {
            context
                .channel
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
        });
        comp
    }
}

type ComputationStream = Pin<Box<dyn Stream<Item = Result<Computation, FieldError>> + Send>>;

#[derive(Deserialize)]
struct UpdateInfo {
    pub id: i32,
    pub email: String,
}

pub(crate) struct Subscription;

#[juniper::graphql_subscription(Context = Context)]
impl Subscription {
    #[graphql(arguments(user_email(name = "email")))]
    async fn computations(ctx: &Context, user_email: String) -> ComputationStream {
        use crate::schema::computations::dsl::*;
        // Create new queue to listen for updates
        let update_queue = ctx
            .channel
            .queue_declare("", QueueDeclareOptions::default(), FieldTable::default())
            .await
            .unwrap();
        // Bind the queue to the computation_updates exchange
        ctx.channel
            .queue_bind(
                &update_queue.name().to_string(),
                "computation_updates",
                "",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();
        let consumer = ctx
            .channel
            .basic_consume(
                &update_queue.name().to_string(),
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();
        let pool_clone = ctx.pool.clone();
        let update_stream = consumer
            .filter_map(|result| async move {
                match result {
                    Ok((_, delivery)) => Some(delivery.data),
                    Err(_) => None,
                }
            })
            .map(move |data| (data, user_email.clone()))
            .filter_map(|(bytes, email_copy)| async move {
                match serde_json::from_slice::<UpdateInfo>(&bytes) {
                    Ok(res) if res.email == email_copy => Some(res),
                    _ => None,
                }
            })
            .map(move |update| {
                let db = pool_clone.get().unwrap();

                Ok(computations
                    .filter(id.eq(update.id))
                    .first::<crate::models::Computation>(&db)
                    .expect(&format!("Error loading computation w/ id {}", update.id)))
            });

        Box::pin(update_stream)
    }
}
