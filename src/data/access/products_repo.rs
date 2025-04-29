use std::env;
use std::sync::Arc;
use async_trait::async_trait;
use mongodb::{Client, Collection};
use mongodb::bson::{doc, Document};
use crate::core::domain::dynamics::products_error::{ProductError, ProductResult};
use crate::core::domain::dynamics::products_repo::ProductRepo;
use crate::core::domain::dynamics::ProductTranslation;

#[derive(Clone, Debug)]
pub struct ProductRepositoryImpl {
    client: Arc<Client>,
}

impl ProductRepositoryImpl {
    pub fn new(client: Arc<Client>) -> Self {
        ProductRepositoryImpl { client }
    }
    fn get_collection(&self, collection: String) -> Collection<Document> {
        let db_name = env::var("MONGO_DATABASE").expect("Variable de entorno MONGO_DATABASE no encontrada");
        self.client.database(&db_name).collection(&collection)
    }
}

#[async_trait]
impl ProductRepo for ProductRepositoryImpl {
    
    async fn create_product(&self, id_product: i64, language: ProductTranslation)-> ProductResult<()> {
        let collection = self.get_collection("products".to_string());
        let mut product_doc = doc! {
            "id_product": id_product,
            "title": language.title,
            "description": language.description,
            "language": language.lang,
        };

        collection
            .insert_one(product_doc.clone())
            .await
            .map_err(ProductError::DbError)?;
        Ok(())
    }
    
}