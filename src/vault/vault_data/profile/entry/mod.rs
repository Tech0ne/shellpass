use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub website: String,
    pub raw_data: String,
}

impl Entry {
    pub fn new() -> Self {
        Self {
            id: Uuid::now_v7(),
            username: String::new(),
            password: String::new(),
            website: String::new(),
            raw_data: String::new(),
        }
    }
}

impl From<crate::app::entry::Entry> for Entry {
    fn from(value: crate::app::entry::Entry) -> Self {
        Self {
            id: Uuid::now_v7(),
            username: value.username,
            password: value.password,
            website: value.website,
            raw_data: value.raw_data,
        }
    }
}
