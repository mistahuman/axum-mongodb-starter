use serde::{Deserialize, Serialize};

use crate::models::exampleitem::ExampleItem;

#[derive(Debug, Deserialize)]
pub struct CreateExampleItem {
    pub title: String,
    pub value: i32,
    pub code: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateExampleItem {
    pub title: Option<String>,
    pub value: Option<i32>,
    pub code: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExampleItemResponse {
    pub id: String,
    pub title: String,
    pub value: i32,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl From<ExampleItem> for ExampleItemResponse {
    fn from(doc: ExampleItem) -> Self {
        Self {
            id: doc.id.map(|oid| oid.to_hex()).unwrap_or_default(),
            title: doc.title,
            value: doc.value,
            code: doc.code,
            description: doc.description,
        }
    }
}
