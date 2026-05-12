
pub mod utils;

#[derive(Serialize, Deserialize)]
pub struct Lang {
    pub id: Option<LangID>,
    pub code: String,
    pub active: bool,
}
