use crate::DbPool;
use crate::models::Computation;
use diesel::prelude::*;
use juniper::{RootNode, EmptyMutation, FieldError};
use std::pin::Pin;
use futures::Stream;

type Schema = RootNode<'static, Query, EmptyMutation<Context>, Subscription>;

pub(crate) fn schema() -> Schema {
    Schema::new (
        Query,
        EmptyMutation::new(),
        Subscription,
    )
}

#[derive(Clone)]
pub(crate) struct Context {
    pub pool: DbPool
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
            .expect(&format!("Error loading computations for user {}", user_email))
    }
}

type ComputationStream = Pin<Box<dyn Stream<Item = Result<Computation, FieldError>> + Send>>;

pub(crate) struct Subscription;

#[juniper::graphql_subscription(Context = Context)]
impl Subscription {
    #[graphql(arguments(user_email(name = "email")))]
    async fn computations(user_email: String) -> ComputationStream {
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