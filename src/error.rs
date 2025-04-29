
use derive_more::IsVariant;
use thiserror::Error;
use crate::core::domain::dynamics::products_error::ProductError;


pub type ServiceResult<T> = Result<T, ServiceError>;

#[derive(Error, IsVariant, Debug)]
pub enum ServiceError
{
    #[error("File Info Error: {0}")]
    ProductsError(#[from] ProductError)
}