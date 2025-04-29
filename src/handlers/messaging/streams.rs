use async_nats::jetstream::{self};
use async_nats::jetstream::stream::{Config, RetentionPolicy};

pub async fn setup_stream(nats_conn: &async_nats::Client) -> Result<(), Box<dyn std::error::Error + Send>> {
    // Obtener el contexto de JetStream
    let jetstream = jetstream::new(nats_conn.clone());

    // Crear el stream con la configuración
    jetstream
        .create_stream(Config {
            name: "product-translation".to_string(), // Nombre del stream
            subjects: vec!["shop-product-translation".to_string()], // Tema asociado
            max_messages: 10_000,                  // Ejemplo de configuración personalizada (opcional)
            retention: RetentionPolicy::Limits, // Política de retención
            ..Default::default()
        })
        .await
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send>)?;


    let stream_info = jetstream.get_stream("product-translation").await;
    match stream_info {
        Ok(_) => {
            println!("Stream 'product-translation' verificado correctamente.");
        }
        Err(err) => {
            eprintln!("Error al verificar el stream: {:?}", err);
            return Err(Box::new(err) as Box<_>);
        }
    }

    println!("Stream 'product-translation' configurado correctamente.");
    Ok(())
}

