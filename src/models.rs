use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[deirve(Debug, Serialize, Deserialize, Clone)]

pub struct Item {
    pub id: Uuid,
    pub name: String,
}
