use crate::core::domain::dynamics::ProductMessageCreate;
use crate::core::domain::dynamics::products_repo::ProductRepo;
use crate::data::access::statics_repo::ProductError;

#[derive(Clone)]
pub struct ProductService<P: ProductRepo>
{
    pub product_repo: P,
}

impl<P: ProductRepo> ProductService<P> where P: ProductRepo 
{
    
    pub async fn new_product(&self, new_product: ProductMessageCreate)-> Result<(), ProductError>{
        
        let id_product = new_product.id_product.parse::<i64>().unwrap();
        
        for language in new_product.translate
        {
            self.product_repo
                .create_product(id_product, language)
                .await
                .map_err(|_| ProductError { message: "Error al insertar el producto en MongoDB".to_string() })?;
        }
        
        Ok(())
    }
    
}