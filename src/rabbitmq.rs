use amiquip::{Connection, ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, QueueDeclareOptions};
use axum::Json;
use std::sync::Arc;

use crate::handlers::insights::db_actions_for_insight_stat;

pub fn connect_to_rabbitmq(pool:Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>) -> amiquip::Result<()> {
    let rabbitmq_url = std::env::var("RABBITMQ_URL").unwrap_or("amqp://guest:guest@localhost:5672".to_string());
    let mut connection = Connection::insecure_open(rabbitmq_url.as_str())?;

    let channel = connection.open_channel(None)?;

    let exchange = channel.exchange_declare(
        amiquip::ExchangeType::Direct,
        "insight_actions_exchange",
        ExchangeDeclareOptions::default(),
    )?;

    let queue = channel.queue_declare("insight_actions_queue", QueueDeclareOptions::default())?;

    queue.bind(&exchange, "routing_key", Default::default())?;

    let consumer = queue.consume(ConsumerOptions::default())?;

    tracing::debug!("Connected to RabbitMQ");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                let body=serde_json::from_str::<serde_json::Value>(&body).expect("Failed to parse JSON");
                println!("({:>3}) Received [{}]", i, body);
                db_actions_for_insight_stat(Json(body),pool.clone());
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}