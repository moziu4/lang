use crate::context::Context;
use crate::utils::catalog_importer;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/catalogs")
            .route("/import", web::post().to(import_catalogs_handler))
    );
}

async fn import_catalogs_handler(context: web::Data<Arc<Context>>) -> impl Responder {
    let client = &*context.client;

    match catalog_importer::import_catalogs(client).await {
        Ok(_) => HttpResponse::Ok().json("Catálogos importados correctamente"),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error al importar catálogos: {:?}", err)),
    }
}
