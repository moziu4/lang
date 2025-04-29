use thiserror::Error;
use mongodb::error::Error as MongoError;

pub type ProductResult<T> = Result<T, ProductError>;

#[derive(Error, Debug)]
pub enum ProductError {
    #[error("Producto no encontrado")]
    ProductNotFound,

    #[error("Data Layer Error: {0}")]
    DbError(#[from] MongoError),

    #[error("Error de validación: {0}")]
    ValidationError(String),

    #[error("Campo faltante en la solicitud: {0}")]
    MissingField(String),

    #[error("Error desconocido")]
    UnknownError,
}
