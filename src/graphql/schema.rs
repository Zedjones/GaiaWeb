use crate::models::Computation;
use crate::DbPool;
use diesel::prelude::*;
use futures::Stream;
use juniper::{EmptyMutation, FieldError, RootNode};
use lapin::{options::*, types::FieldTable};
use serde::Deserialize;
use std::pin::Pin;

type Schema = RootNode<'static, Query, EmptyMutation<Context>, Subscription>;

pub(crate) fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), Subscription)
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
        // We have to make a static reference that gets copied instead of doing &user_email in the closure
        // because doing that moves the original variable and that doesn't work with an iterator
        // FIXME: Make it so that the user email can be cloned and moved into the closure
        let email_ref = user_email.clone();
        let pool_clone = ctx.pool.clone();
        let update_stream = consumer
            .filter_map(|result| async move {
                match result {
                    Ok((_, delivery)) => Some(delivery.data),
                    Err(_) => None,
                }
            })
            .filter_map(|bytes| async move {
                match serde_json::from_slice::<UpdateInfo>(&bytes) {
                    Ok(res) if res.email == email_ref => Some(res),
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
