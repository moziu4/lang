use async_trait::async_trait;
use crate::core::domain::statics::languages_type::Language;
use crate::data::access::statics_repo::ProductError;

#[async_trait]
pub trait LanguagesRepo {
    async fn get_all_languages(&self) -> Result<Vec<Language>, ProductError>;
}
