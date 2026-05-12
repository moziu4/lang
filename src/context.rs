use std::sync::Arc;

use crate::utils::cache::RedisCache;
use mongodb::Client;
use crate::data::access::statics_repo::StaticRepositoryImpl;
use crate::data::access::languages_repo::LanguagesRepositoryImpl;

#[derive(Clone)]
pub struct Context {
    pub client: Arc<Client>,
    static_repo: Arc<StaticRepositoryImpl>,
    languages_repo: Arc<LanguagesRepositoryImpl>,
    redis_cache: Arc<RedisCache>,
}

impl Context {
    pub fn new(client: Client, redis_cache: RedisCache) -> Self {
        let arc_client = Arc::new(client);
        Context {
            client: arc_client.clone(),
            static_repo: Arc::new(StaticRepositoryImpl::new(arc_client.clone())),
            languages_repo: Arc::new(LanguagesRepositoryImpl::new(arc_client.clone())),
            redis_cache: Arc::new(redis_cache),
        }
    }

    pub fn get_static_repo(&self) -> Arc<StaticRepositoryImpl> {
        Arc::clone(&self.static_repo)
    }

    pub fn get_languages_repo(&self) -> Arc<LanguagesRepositoryImpl> {
        Arc::clone(&self.languages_repo)
    }

    pub fn get_redis_cache(&self) -> Arc<RedisCache> {
        Arc::clone(&self.redis_cache)
    }
}
