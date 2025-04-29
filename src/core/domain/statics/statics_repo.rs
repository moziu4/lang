use std::collections::HashMap;
use async_trait::async_trait;
use warp::reject;


#[async_trait]
pub trait StaticRepo {
    async fn get_translation(&self, lang: String, group: String) -> Result<HashMap<String, String>, reject::Rejection>;


}