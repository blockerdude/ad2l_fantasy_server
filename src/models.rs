use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Conference {
    pub id: i32,
    pub name: String,
    pub description: String,
}