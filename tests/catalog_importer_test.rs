use lang::utils::catalog_importer;
use mongodb::Client;
use std::env;

#[tokio::test]
async fn test_import_catalogs_local() {
    // Necesitamos una instancia de MongoDB para este test, 
    // pero como es un entorno de CI/CD o local, 
    // asumimos que MONGO_URI está disponible o lo saltamos.
    let mongo_uri = env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    
    // Intentar conectar, si falla, saltamos el test
    let client = match Client::with_uri_str(&mongo_uri).await {
        Ok(c) => c,
        Err(_) => {
            println!("Saltando test de importación porque MongoDB no está disponible.");
            return;
        }
    };

    // Forzar ruta local para el test
    env::set_var("CATALOGS_PATH", "tests/fixtures");
    env::set_var("MONGO_DATABASE", "lang_test_db");
    env::set_var("CATALOGS_TO_IMPORT", "languages.json,perms.json,roles.json");

    let result = catalog_importer::import_catalogs(&client).await;
    
    assert!(result.is_ok(), "La importación debería ser exitosa");
    
    // Verificar que las colecciones existen y tienen datos
    let db = client.database("lang_test_db");
    
    let perms_count = db.collection::<mongodb::bson::Document>("perms")
        .count_documents(mongodb::bson::doc! {}).await.unwrap();
    assert!(perms_count > 0, "La colección perms debería tener elementos");

    let roles_count = db.collection::<mongodb::bson::Document>("roles")
        .count_documents(mongodb::bson::doc! {}).await.unwrap();
    assert!(roles_count > 0, "La colección roles debería tener elementos");

    let langs_count = db.collection::<mongodb::bson::Document>("languages")
        .count_documents(mongodb::bson::doc! {}).await.unwrap();
    assert!(langs_count > 0, "La colección languages debería tener elementos");

    // Limpieza
    db.drop().await.ok();
}
