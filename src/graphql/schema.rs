use crate::models::Computation;
use crate::DbPool;
use diesel::prelude::*;
use futures::Stream;
use juniper::{EmptyMutation, FieldError, RootNode};
use lapin::{options::*, types::FieldTable};
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

pub(crate) struct Subscription;

#[juniper::graphql_subscription(Context = Context)]
impl Subscription {
    #[graphql(arguments(user_email(name = "email")))]
    async fn computations(ctx: &Context, user_email: String) -> ComputationStream {
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
        let stream = tokio::time::interval(std::time::Duration::from_secs(5)).map(move |_| {
            Err(FieldError::new(
                "Some field error from handler",
                juniper::Value::Scalar(juniper::DefaultScalarValue::String(
                    "some additional strng".to_string(),
                )),
            ))
        });

        Box::pin(stream)
    }
}
