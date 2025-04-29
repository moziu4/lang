use crate::context::Context;
use crate::core::operation::statics_ops::{StaticService};
use actix_web::web::{Path};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::sync::Arc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/statics")
        .route("/{group}", web::get().to(get_base_translation))
    );
}


async fn get_base_translation(context: web::Data<Arc<Context>>, req: HttpRequest, path: Path<String>) -> impl Responder {
    let static_service = StaticService {
        static_repo: context.get_ref().get_static_repo(),
        redis_cache: context.get_ref().get_redis_cache(),
    };
    let lang = req
        .headers()
        .get("Accept-Language")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("es");

    let group = path.into_inner();

    match static_service.load_translates(lang.parse().unwrap(), group ).await {
        Ok(translation) => HttpResponse::Ok().json(translation),
        Err(err) => HttpResponse::InternalServerError().json(err.message),
    }
}



