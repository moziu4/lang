use std::{env, io, sync::Arc};
use actix_cors::Cors;
use actix_web::{
    dev::RequestHead,
    http::header::{self, HeaderValue},
    web,
    App,
    HttpServer,
};
use lang::{context::Context, db::connect_to_db, handlers::http};
use env_logger::Env;
use tokio::spawn;
use lang::utils::cache::RedisCache;
use futures::{try_join, TryFutureExt};
use lang::core::operation::products_ops::ProductService;
use lang::data::access::products_repo::ProductRepositoryImpl;
use lang::handlers::messaging;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let client = connect_to_db().await;
    let product_repo = ProductRepositoryImpl::new(Arc::new(client.clone()));

    let redis_cache = RedisCache::new(&*env::var("REDIS_URI").unwrap())
        .expect("Error al conectar a Redis");

    // Crear una referencia contenedora (Arc) para el contexto
    let context = Arc::new(Context::new(client, redis_cache));

    // Clonar el contexto para el servidor HTTP antes de moverlo a la clausura
    let http_context = context.clone();

    // Configurar el servidor HTTP
    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin_fn(|origin: &HeaderValue, _req_head: &RequestHead| {
                        if let Ok(origin_str) = origin.to_str() {
                            origin_str == env::var("URL_FRONT_DEV").unwrap()
                                || origin_str == env::var("URL_FRONT").unwrap()
                        } else {
                            false
                        }
                    })
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
                    .max_age(3600),
            )
            .app_data(web::Data::new(http_context.clone())) // Se move `http_context` aquí
            .configure(http::statics::statics_routes::config)
    })
        .bind(env::var("HTTP_BIND").unwrap().to_string())?
        .run();
    
    let product_service = ProductService {
        product_repo,
    };

    let nats_listener_context = context.clone();
    let nats_listener = spawn(async move {
        if let Err(err) = messaging::message_config(nats_listener_context, product_service).await {
            eprintln!("Error en la configuración de mensajes: {:?}", err);
        }
    })
        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("Task join error: {}", err)));


    try_join!(http_server, nats_listener).map(|_| ())

}







