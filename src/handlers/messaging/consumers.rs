use async_nats::jetstream::{self, consumer};
use async_nats::jetstream::consumer::AckPolicy;

pub async fn setup_consumer(nats_conn: &async_nats::Client) -> Result<(), Box<dyn std::error::Error + Send>> {
    // Obtener el contexto de JetStream
    let jetstream = jetstream::new(nats_conn.clone());
    let stream = jetstream.get_stream("product-translation")
        .await
        .map_err(|err| -> Box<dyn std::error::Error + Send> { Box::new(err) })?;
    
    // Crear un consumidor asociado al stream "translations_stream"
    let _consumer = stream
        .create_consumer(consumer::pull::Config {
            durable_name: Some("lang_product_consumer".to_string()),
            ack_policy: AckPolicy::Explicit,

            ..Default::default()
        })
        .await
        .map_err(|err| -> Box<dyn std::error::Error + Send> { Box::new(err) })?;
     // Invocar la operación asíncrona correctamente

    println!("Consumidor 'lang_product_consumer' configurado en el stream 'product-translation'.");

    Ok(())
}




