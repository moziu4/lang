use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Language {
    #[serde(rename = "id")]
    pub id: i32,
    pub code: String,
    pub name: String,
}
