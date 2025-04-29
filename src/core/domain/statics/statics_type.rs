use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StaticTranslate
{
    pub data:  HashMap<String, String>,
}


