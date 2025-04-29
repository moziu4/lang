pub mod products_repo;
pub mod products_error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProductTranslation {
    pub title: String,
    pub description: String,
    pub lang: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductMessageCreate {
    pub id_product: String,
    pub translate: Vec<ProductTranslation>,
}
