use std::env;
use std::fs;
use std::path::Path;
use mongodb::Client;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CatalogItem {
    #[serde(rename = "id")]
    pub catalog_id: i32,
    #[serde(flatten)]
    pub data: HashMap<String, Value>,
}

pub async fn import_catalogs(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let catalogs_path = env::var("CATALOGS_PATH").unwrap_or_else(|_| "tests/fixtures".to_string());
    
    // Lista de catálogos a importar, configurable desde variable de entorno (separados por coma)
    let catalogs_to_import_str = env::var("CATALOGS_TO_IMPORT").unwrap_or_else(|_| "languages.json".to_string());
    let catalogs_to_import: Vec<&str> = catalogs_to_import_str.split(',').collect();
    
    let db_name = env::var("MONGO_DATABASE").unwrap_or_else(|_| "lang_db".to_string());
    let db = client.database(&db_name);

    for catalog_file in catalogs_to_import {
        let collection_name = catalog_file.replace(".json", "");
        println!("Intentando importar catálogo: {}", collection_name);
        
        let content = match load_catalog_content(&catalogs_path, catalog_file).await {
            Some(content) => content,
            None => {
                println!("No se pudo encontrar el catálogo {} en ninguna de las rutas.", catalog_file);
                continue;
            }
        };

        let items: Vec<CatalogItem> = match serde_json::from_str(&content) {
            Ok(items) => items,
            Err(e) => {
                println!("Error al deserializar {}: {:?}", catalog_file, e);
                continue;
            }
        };

        // Limpiar colección
        let collection = db.collection::<CatalogItem>(&collection_name);
        collection.drop().await?;
        println!("Colección {} vaciada.", collection_name);

        if !items.is_empty() {
            collection.insert_many(items).await?;
            println!("Importados {} elementos en {}.", collection_name, collection_name);
        }
    }

    Ok(())
}

async fn load_catalog_content(base_path: &str, file_name: &str) -> Option<String> {
    // 1. Carga Remota (Prioridad Alta)
    if base_path.starts_with("http") {
        println!("Intentando carga remota desde: {}/{}", base_path, file_name);
        let url = format!("{}/{}", base_path.trim_end_matches('/'), file_name);
        let client = reqwest::Client::new();
        let response = client.get(&url)
            .header(USER_AGENT, "my-service-importer")
            .send()
            .await;

        match response {
            Ok(res) if res.status().is_success() => {
                match res.text().await {
                    Ok(text) => {
                        println!("Archivo {} descargado exitosamente.", file_name);
                        return Some(text);
                    }
                    Err(e) => println!("Error al leer el cuerpo de la respuesta remota: {:?}", e),
                }
            }
            Ok(res) => println!("Carga remota falló con status: {}", res.status()),
            Err(e) => println!("Error de red en carga remota: {:?}", e),
        }
        println!("Pasando a carga local (Fallback)...");
    }

    // 2. Carga Local Parametrizada
    println!("Buscando localmente en: {}/{}", base_path, file_name);
    let local_path = Path::new(base_path).join(file_name);
    if let Ok(content) = fs::read_to_string(&local_path) {
        println!("Archivo {} encontrado en ruta parametrizada.", file_name);
        return Some(content);
    }

    // 3. Búsqueda Exhaustiva por Rutas de Fallback
    let fallback_paths = vec![
        "/opt/catalogs/",
        "/opt/",
        "../catalogs/",
        "catalogs/",
        "tests/fixtures/",
    ];

    for path_str in fallback_paths {
        let path = Path::new(path_str).join(file_name);
        println!("Buscando en ruta de fallback: {:?}", path);
        if let Ok(content) = fs::read_to_string(&path) {
            println!("Archivo {} encontrado en ruta de fallback: {:?}", file_name, path);
            return Some(content);
        }
    }

    None
}
