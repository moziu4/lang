use crate::core::domain::statics::statics_repo::StaticRepo;
use async_trait::async_trait;
use mongodb::{ Client};
use std::collections::HashMap;
use std::{env, fs};
use std::path::Path;
use std::sync::Arc;
use warp::{reject, Rejection};
use dotenv::dotenv;

#[derive(Debug)]
pub struct ProductError {
    pub message: String,
}

impl reject::Reject for ProductError {}

#[derive(Clone, Debug)]
pub struct StaticRepositoryImpl {
    _client: Arc<Client>,
}

impl StaticRepositoryImpl {
    pub fn new(client: Arc<Client>) -> Self {
        StaticRepositoryImpl { _client: client, }
    }
}

#[async_trait]
impl StaticRepo for StaticRepositoryImpl {
    async fn get_translation(&self, lang: String, group: String) -> Result<HashMap<String, String>, Rejection> {
        dotenv().ok(); // Carga el archivo .env si está disponible

        // Obtén la ruta base desde una variable de entorno
        let base_path = env::var("TRANSLATIONS_DIR").unwrap_or_else(|_| "src/translation".to_string());

        // Construye la ruta completa al archivo de traducción
        let file_path = Path::new(&base_path)
            .join(&group)
            .join(format!("{}.json", lang));

        let file_path_str = file_path.to_str().unwrap();

        // Resto del código para leer el archivo y manejar errores
        let file_content = fs::read_to_string(&file_path).map_err(|err| {
            warp::reject::custom(ProductError {
                message: format!("Error leyendo el archivo {}: {:?}", file_path_str, err),
            })
        })?;

        let translations: HashMap<String, String> = serde_json::from_str(&file_content).map_err(|err| {
            warp::reject::custom(ProductError {
                message: format!("Error deserializando el archivo {} como JSON: {:?}", file_path_str, err),
            })
        })?;


        Ok(translations)
    }
}
