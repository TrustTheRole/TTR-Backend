use amiquip::{Connection, ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, QueueDeclareOptions};
use axum::Json;
use log::{debug, info};
use std::sync::Arc;
use tokio::task;

use crate::{handlers::insights::db_actions_for_insight_stat, utils::dispatch_email};

pub fn connect_rabbitmq_services(pool: Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>) {

    tokio::spawn(async move {
        let result = connect_rabbitmq_insights_handler(pool);
        if let Err(e) = result {
            log::error!("Error connecting to RabbitMq-Insights service: {:?}", e);
        }
    });

    tokio::spawn(async move {
        let result = rabbitmq_email_handler().await;
        if let Err(e) = result {
            log::error!("Error connecting to RabbitMq-Email service: {:?}", e);
        }
    });
}

pub fn connect_rabbitmq_insights_handler(pool: Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>) -> amiquip::Result<()> {
    let rabbitmq_url = std::env::var("RABBITMQ_URL").unwrap_or("amqp://guest:guest@localhost:5672".to_string());
    let mut connection = Connection::insecure_open(rabbitmq_url.as_str())?;

    let channel = connection.open_channel(None)?;

    let insight_actions_exchange = channel.exchange_declare(
        amiquip::ExchangeType::Direct,
        "insight_actions_exchange",
        ExchangeDeclareOptions::default(),
    )?;

    let insight_actions_queue = channel.queue_declare("insight_actions_queue", QueueDeclareOptions::default())?;

    insight_actions_queue.bind(&insight_actions_exchange, "routing_key", Default::default())?;

    let insight_actions_consumer = insight_actions_queue.consume(ConsumerOptions::default())?;

    info!("RabbitMQ-Insights service connected");

    for (i, message) in insight_actions_consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                let body = serde_json::from_str::<serde_json::Value>(&body).expect("Failed to parse JSON");
                debug!("({:>3}) Received [{}]", i, body);
                db_actions_for_insight_stat(Json(body), pool.clone());
                insight_actions_consumer.ack(delivery)?;
            }
            other => {
                debug!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}

pub async fn rabbitmq_email_handler() -> amiquip::Result<()> {
    let rabbitmq_url = std::env::var("RABBITMQ_URL").unwrap_or("amqp://guest:guest@localhost:5672".to_string());
    let mut connection = Connection::insecure_open(rabbitmq_url.as_str())?;

    let channel = connection.open_channel(None)?;
    let email_actions_exchange = channel.exchange_declare(
        amiquip::ExchangeType::Direct,
        "email_actions_exchange",
        ExchangeDeclareOptions::default(),
    )?;

    let email_actions_queue = channel.queue_declare("email_actions_queue", QueueDeclareOptions::default())?;

    email_actions_queue.bind(&email_actions_exchange, "routing_key", Default::default())?;

    let email_actions_consumer = email_actions_queue.consume(ConsumerOptions::default())?;
    
    info!("RabbitMQ-Email service connected");

    task::block_in_place(move || {
        for (i, message) in email_actions_consumer.receiver().iter().enumerate() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let body = String::from_utf8_lossy(&delivery.body);
                    let body = serde_json::from_str::<serde_json::Value>(&body).expect("Failed to parse JSON");

                    let fullname = body["user_name"].as_str().unwrap();
                    let email = body["user_email"].as_str().unwrap();
                    let message = body["message"].as_str().unwrap();
                    let email_subject = body["subject"].as_str().unwrap();
                    let html_content = body["html_content"].as_str().unwrap();

                    debug!("({:>3}) Received [{}]", i, body);
                    task::block_in_place(move || {
                        tokio::runtime::Runtime::new().unwrap().block_on(async {
                            dispatch_email(fullname, email, message, email_subject.to_string(), html_content).await;
                        })
                    });
                    email_actions_consumer.ack(delivery)?;
                }
                other => {
                    debug!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }
        Ok(())
    })
}
