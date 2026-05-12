use std::collections::HashMap;
use std::sync::Arc;
use crate::core::domain::statics::statics_repo::StaticRepo;
use crate::data::access::statics_repo::ProductError;
use crate::utils::cache::RedisCache;

pub struct StaticService<P: StaticRepo>
{
    pub static_repo: Arc<P>,
    pub redis_cache: Arc<RedisCache>
}

impl<P: StaticRepo> StaticService<P> where P: StaticRepo
{
    pub async fn load_translates(&self, lang: String, group: String) -> Result<HashMap<String, String>, ProductError> {
        let cache_key = group.clone() + "_" + &lang;

        if let Some(products) = self.redis_cache.get_translates(cache_key.as_str()).await.map_err(|err| {
            ProductError {
                message: format!("Error obteniendo productos de Redis: {:?}", err),
            }
        })? {
            return Ok(products);
        }
        let products = self.static_repo
            .get_translation(lang.clone(), group.clone())
            .await
            .map_err(|product_err| {
                ProductError {
                    message: format!("Error processing request: {:?}", product_err),
                }
            })?;

        // Guardar en caché si se obtuvo del repo
        let _ = self.redis_cache.set_products(&cache_key, products.clone(), 3600).await;

        Ok(products)

    }


}