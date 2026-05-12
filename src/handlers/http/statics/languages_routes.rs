use crate::context::Context;
use crate::core::operation::languages_ops::LanguagesService;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/languages")
            .route("", web::get().to(get_all_languages))
    );
}

async fn get_all_languages(context: web::Data<Arc<Context>>) -> impl Responder {
    let languages_service = LanguagesService {
        languages_repo: context.get_ref().get_languages_repo(),
    };

    match languages_service.get_all_languages().await {
        Ok(languages) => HttpResponse::Ok().json(languages),
        Err(err) => HttpResponse::InternalServerError().json(err.message),
    }
}
