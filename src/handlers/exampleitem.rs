use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ReturnDocument,
};
use serde_json::{Value, json};

use crate::{
    AppState,
    models::exampleitem::ExampleItem,
    schemas::exampleitem::{CreateExampleItem, ExampleItemResponse, UpdateExampleItem},
};

const COLLECTION: &str = "exampleitems";

type ApiError = (StatusCode, Json<Value>);
type ApiResult<T> = Result<T, ApiError>;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/exampleitems/", get(list_exampleitems).post(create_exampleitem))
        .route("/exampleitems/{id}", get(get_exampleitem).patch(update_exampleitem).delete(delete_exampleitem))
}

async fn list_exampleitems(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<ExampleItemResponse>>> {
    let col = state.db.collection::<ExampleItem>(COLLECTION);
    let mut cursor = col.find(doc! {}).await.map_err(internal_error)?;
    let mut items = Vec::new();
    while cursor.advance().await.map_err(internal_error)? {
        items.push(ExampleItemResponse::from(cursor.deserialize_current().map_err(internal_error)?));
    }
    Ok(Json(items))
}

async fn get_exampleitem(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> ApiResult<Json<ExampleItemResponse>> {
    let oid = parse_oid(&id)?;
    let col = state.db.collection::<ExampleItem>(COLLECTION);
    let item = col
        .find_one(doc! {"_id": oid})
        .await
        .map_err(internal_error)?
        .ok_or_else(|| not_found(&id))?;
    Ok(Json(ExampleItemResponse::from(item)))
}

async fn create_exampleitem(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateExampleItem>,
) -> ApiResult<(StatusCode, Json<ExampleItemResponse>)> {
    let doc = ExampleItem {
        id: Some(ObjectId::new()),
        title: payload.title,
        value: payload.value,
        code: payload.code,
        description: payload.description,
    };
    let col = state.db.collection::<ExampleItem>(COLLECTION);
    col.insert_one(&doc).await.map_err(internal_error)?;
    Ok((StatusCode::CREATED, Json(ExampleItemResponse::from(doc))))
}

async fn update_exampleitem(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateExampleItem>,
) -> ApiResult<Json<ExampleItemResponse>> {
    let oid = parse_oid(&id)?;
    let mut fields = doc! {};
    if let Some(t) = payload.title { fields.insert("title", t); }
    if let Some(v) = payload.value { fields.insert("value", v); }
    if let Some(c) = payload.code { fields.insert("code", c); }
    if let Some(d) = payload.description { fields.insert("description", d); }
    if fields.is_empty() {
        return Err(bad_request("no fields to update"));
    }
    let col = state.db.collection::<ExampleItem>(COLLECTION);
    let item = col
        .find_one_and_update(doc! {"_id": oid}, doc! {"$set": fields})
        .return_document(ReturnDocument::After)
        .await
        .map_err(internal_error)?
        .ok_or_else(|| not_found(&id))?;
    Ok(Json(ExampleItemResponse::from(item)))
}

async fn delete_exampleitem(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> ApiResult<StatusCode> {
    let oid = parse_oid(&id)?;
    let col = state.db.collection::<ExampleItem>(COLLECTION);
    let result = col.delete_one(doc! {"_id": oid}).await.map_err(internal_error)?;
    if result.deleted_count == 0 {
        return Err(not_found(&id));
    }
    Ok(StatusCode::NO_CONTENT)
}

fn parse_oid(id: &str) -> Result<ObjectId, ApiError> {
    ObjectId::parse_str(id).map_err(|_| bad_request("invalid id"))
}

fn internal_error<E: std::fmt::Display>(err: E) -> ApiError {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()})))
}

fn bad_request(msg: &str) -> ApiError {
    (StatusCode::BAD_REQUEST, Json(json!({"error": msg})))
}

fn not_found(id: &str) -> ApiError {
    (StatusCode::NOT_FOUND, Json(json!({"error": format!("ExampleItem {} not found", id)})))
}
