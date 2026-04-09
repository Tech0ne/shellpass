mod field;

use field::Field;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub website: String,
    pub custom_fields: Vec<Field>,
}

impl Entry {
    pub fn new() -> Self {
        Self {
            id: Uuid::now_v7(),
            username: String::new(),
            password: String::new(),
            website: String::new(),
            custom_fields: Vec::new(),
        }
    }
}
