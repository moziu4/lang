use std::sync::Arc;
use crate::core::domain::statics::languages_repo::LanguagesRepo;
use crate::core::domain::statics::languages_type::Language;
use crate::data::access::statics_repo::ProductError;

pub struct LanguagesService<R: LanguagesRepo> {
    pub languages_repo: Arc<R>,
}

impl<R: LanguagesRepo> LanguagesService<R> {
    pub async fn get_all_languages(&self) -> Result<Vec<Language>, ProductError> {
        self.languages_repo.get_all_languages().await
    }
}
