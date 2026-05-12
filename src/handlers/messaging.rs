use std::sync::Arc;
use crate::context::Context;
use crate::core::domain::dynamics::products_repo::ProductRepo;
use crate::core::operation::products_ops::ProductService;
use crate::handlers::messaging::streams::setup_stream;
use crate::handlers::messaging::consumers::setup_consumer;
use crate::handlers::messaging::products::consume_with_consumer;

pub mod products;
pub mod streams;
pub mod consumers;

pub async fn message_config( context: Arc<Context>, product_service: ProductService<impl ProductRepo + Send + Sync + 'static>,) -> Result<(), Box<dyn std::error::Error + Send>> {
    // Conectar a NATS
    let nats_conn = async_nats::connect("nats://nats:4222").await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;

    // Configurar el stream y el consumidor
    setup_stream(&nats_conn).await?;
    setup_consumer(&nats_conn).await?;

    // Consumir mensajes
    consume_with_consumer(context, product_service, &nats_conn).await?;

    Ok(())
}


