use std::collections::HashMap;
use redis::{
    AsyncCommands, Client, RedisError
};
use serde_json;
use std::sync::Arc;
use crate::core::domain::statics::statics_type::StaticTranslate;
use crate::utils::cache_error::CacheError;

// Estructura para manejar Redis
#[derive(Clone)]
pub struct RedisCache {
    client: Arc<Client>,
}

impl RedisCache {
    pub fn new(connection_string: &str) -> Result<Self, RedisError> {
        let client = Client::open(connection_string)?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub async fn get_translates(&self, cache_key: &str) -> Result<Option<HashMap<String, String>>, CacheError> {
        let mut con = self.client.get_multiplexed_async_connection().await?;

        // Intenta obtener el valor asociado a la clave
        let cached_value: Option<String> = con.get(cache_key).await?;
        if let Some(value) = cached_value {
            // Intenta deserializar; el operador `?` convierte automáticamente los errores a `CacheError`
            let products: HashMap<String, String> = serde_json::from_str(&value)?;
            return Ok(Some(products));
        }

        Ok(None) // Si no hay datos almacenados en la caché
    }

    pub async fn set_products(
        &self,
        cache_key: &str,
        products: HashMap<String, String>,
        ttl: usize,
    ) -> Result<(), CacheError> {
        let mut con = self.client.get_multiplexed_async_connection().await?;

        // Serializa los productos a un String en formato JSON
        let serialized = serde_json::to_string(&products)?;

        // Almacena los datos en Redis con el TTL especificado
        con.set_ex(cache_key, serialized, ttl as u64).await?;
        Ok(())
    }

    pub async fn invalidate_cache(&self, cache_key: &str) -> Result<(), RedisError> {
        let mut con = self.client.get_async_connection().await?;
        let _: () = con.del(cache_key).await?; // Eliminamos la clave específica
        Ok(())
    }
}