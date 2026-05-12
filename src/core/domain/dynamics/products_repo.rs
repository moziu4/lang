use async_trait::async_trait;
use crate::core::domain::dynamics::products_error::ProductResult;
use crate::core::domain::dynamics::ProductTranslation;

#[async_trait]
pub trait ProductRepo {
    async fn create_product(&self,  id_product: i64, language: ProductTranslation)-> ProductResult<()>;
}