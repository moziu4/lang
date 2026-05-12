use std::env;
use std::sync::Arc;
use async_trait::async_trait;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use futures::stream::StreamExt;
use crate::core::domain::statics::languages_type::Language;
use crate::core::domain::statics::languages_repo::LanguagesRepo;
use crate::data::access::statics_repo::ProductError;

#[derive(Clone, Debug)]
pub struct LanguagesRepositoryImpl {
    client: Arc<Client>,
}

impl LanguagesRepositoryImpl {
    pub fn new(client: Arc<Client>) -> Self {
        LanguagesRepositoryImpl { client }
    }
    
    fn get_collection(&self) -> Collection<Language> {
        let db_name = env::var("MONGO_DATABASE").expect("Variable de entorno MONGO_DATABASE no encontrada");
        self.client.database(&db_name).collection("languages")
    }
}

#[async_trait]
impl LanguagesRepo for LanguagesRepositoryImpl {
    async fn get_all_languages(&self) -> Result<Vec<Language>, ProductError> {
        let collection = self.get_collection();
        let mut cursor = collection.find(doc! {}).await.map_err(|e| ProductError {
            message: format!("Error al buscar idiomas en BD: {:?}", e),
        })?;

        let mut languages = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(lang) => languages.push(lang),
                Err(e) => return Err(ProductError {
                    message: format!("Error al obtener idioma del cursor: {:?}", e),
                }),
            }
        }
        
        Ok(languages)
    }
}
