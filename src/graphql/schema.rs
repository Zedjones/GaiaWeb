use crate::DbPool;
use crate::models::Computation;
use diesel::prelude::*;
use juniper::{RootNode, EmptyMutation, EmptySubscription};

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub(crate) fn schema() -> Schema {
    Schema::new (
        Query,
        EmptyMutation::new(),
        EmptySubscription::new(),
    )
}

pub(crate) struct Context {
    pub pool: DbPool
}

impl juniper::Context for Context {}

pub(crate) struct Query;

#[juniper::graphql_object(
    Context = Context,
)]
impl Query {
    #[graphql(
        arguments(
            user_email (
                name = "email"
            )
        )
    )]
    fn get_computations(context: &Context, user_email: String) -> Option<Vec<Computation>> {
        use crate::schema::computations::dsl::*;
        
        let db = context.pool.get().unwrap();

        let user_computations = computations
            .filter(email.eq(&user_email))
            .load::<crate::models::Computation>(&db)
            .expect(&format!("Error loading computations for user {}", user_email));
    
        if user_computations.len() == 0 {
            None
        }
        else {
            Some(user_computations)
        }
    }
}