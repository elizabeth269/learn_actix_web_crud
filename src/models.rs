use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct Item {
    pub id: Uuid,
    pub name: String,
}
