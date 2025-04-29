use std::sync::Arc;
use async_nats::jetstream::consumer::pull;
use futures::{StreamExt, TryStreamExt};
use crate::context::Context;
use crate::core::domain::dynamics::ProductMessageCreate;
use crate::core::domain::dynamics::products_repo::ProductRepo;
use crate::core::operation::products_ops::ProductService;

pub async fn consume_with_consumer<P>(
    _context: Arc<Context>,
    product_service: ProductService<P>,
    nats_client: &async_nats::Client,
) -> Result<(), Box<dyn std::error::Error + Send>>
where
    P: ProductRepo + Send + Sync + 'static,
{
    // Crear el contexto JetStream usando el cliente de NATS
    let jetstream = async_nats::jetstream::new(nats_client.clone());

    // Conectar al stream externo "shop-product-translation"
    let stream = jetstream.get_stream("product-translation").await.map_err(|err| {
        eprintln!("No se pudo obtener el stream 'product-translation': {:?}", err);
        Box::new(err) as Box<dyn std::error::Error + Send>
    })?;

    // Consumir mensajes directamente del consumidor "translations_consumer"
    let consumer: async_nats::jetstream::consumer::Consumer<pull::Config> =
        stream.get_consumer("lang_product_consumer").await
            .map_err(|err| Box::<dyn std::error::Error + Send>::from(err))?;




    let mut consumer_messages = consumer
        .messages()
        .await
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send>)?
        .boxed();



    // Procesar cada mensaje del consumidor
    while let Some(message) = consumer_messages.next().await {
        match message {
            Ok(mut msg) => {
                println!("Mensaje recibido: {:?}", msg);

                // Intentar deserializar el mensaje recibido
                match serde_json::from_slice::<ProductMessageCreate>(&msg.payload) {
                    Ok(product_message) => {
                        // Llama a la operación de creación del producto
                        match product_service.new_product(product_message).await {
                            Ok(_) => {
                                println!("Producto procesado y creado exitosamente.");
                                // Envía el ACK porque todo salió bien
                                if let Err(ack_err) = msg.ack().await {
                                    eprintln!("Error al enviar ACK: {:?}", ack_err);
                                }
                            }
                            Err(err) => {
                                eprintln!("Error al procesar el producto: {:?}", err);
                                // Dependiendo de tu lógica, decide si enviar un ACK aquí o no
                                // Aquí se decide enviar el ACK en caso de error para evitar reintentos infinitos.
                                if let Err(ack_err) = msg.ack().await {
                                    eprintln!("Error al enviar ACK después de fallo: {:?}", ack_err);
                                }
                            }
                        }
                    }
                    Err(err) => {
                        // Si falla la deserialización, no se puede procesar, pero asegurarte de enviar el ACK
                        eprintln!("Error deserializando el mensaje: {:?}", err);
                        if let Err(ack_err) = msg.ack().await {
                            eprintln!("Error al enviar ACK después de error de deserialización: {:?}", ack_err);
                        }
                    }
                }
            }
            Err(err) => {
                // Error al recibir el mensaje desde el stream
                eprintln!("Error al recibir mensaje: {:?}", err);
            }
        }
    }



    Ok(())
}


