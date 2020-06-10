use crate::models::Computation;
use crate::routes::{PutSettings, QueueMessage};
use crate::DbPool;
use async_graphql::{Context, FieldError};
use diesel::prelude::*;
use futures::{Stream, StreamExt};
use lapin::{options::*, types::FieldTable, BasicProperties, Channel};
use log::info;
use serde::Deserialize;

pub(crate) type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

pub(crate) fn schema(send_chan: Channel, pool: DbPool) -> Schema {
    async_graphql::Schema::build(Query, Mutation, Subscription)
        .data(send_chan)
        .data(pool)
        .finish()
}

pub(crate) struct Query;

#[async_graphql::Object]
impl Query {
    async fn get_computations(
        &self,
        context: &Context<'_>,
        #[arg(name = "email")] user_email: String,
    ) -> Vec<Computation> {
        use crate::schema::computations::dsl::*;

        let db = context.data::<DbPool>().get().unwrap();

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

#[async_graphql::Object]
impl Mutation {
    async fn add_computation(
        &self,
        context: &Context<'_>,
        settings: PutSettings,
        csv_file: String,
    ) -> Computation {
        use crate::models::NewComputation;
        let new_comp = NewComputation {
            email: settings.email.clone(),
            title: settings.title.clone(),
            csv_file: base64::decode(csv_file).unwrap(),
        };
        let comp = new_comp.insert_computation(context.data::<DbPool>());
        info!("Created computation with id: {}", comp.id);
        let queue_message = QueueMessage {
            data_id: comp.id,
            settings: &settings,
        };
        context
            .data::<Channel>()
            .basic_publish(
                "",
                "gaia_input",
                BasicPublishOptions::default(),
                serde_json::to_vec(&queue_message).unwrap(),
                BasicProperties::default(),
            )
            .await
            .unwrap();
        context
            .data::<Channel>()
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
        comp
    }
}

#[derive(Deserialize)]
struct UpdateInfo {
    pub id: i32,
    pub email: String,
}

pub(crate) struct Subscription;

#[async_graphql::Subscription]
impl Subscription {
    async fn computations(
        &self,
        ctx: &Context<'_>,
        #[arg(name = "email")] user_email: String,
    ) -> impl Stream<Item = Result<Computation, FieldError>> {
        use crate::schema::computations::dsl::*;
        // Create new queue to listen for updates
        let update_queue = ctx
            .data::<Channel>()
            .queue_declare("", QueueDeclareOptions::default(), FieldTable::default())
            .await
            .unwrap();
        // Bind the queue to the computation_updates exchange
        ctx.data::<Channel>()
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
            .data::<Channel>()
            .basic_consume(
                &update_queue.name().to_string(),
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();
        let pool_clone = ctx.data::<DbPool>().clone();
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

        update_stream
    }
}
