use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TicketStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    id: Uuid,
    title: String,
    description: String,
    status: TicketStatus,
}

#[derive(Debug, Deserialize)]
pub struct CreateTicket {
    title: String,
    description: String,
}

#[derive(Debug, Deserialize)]
pub struct PatchTicket {
    title: Option<String>,
    description: Option<String>,
    status: Option<TicketStatus>,
}

pub type TicketStore = Arc<RwLock<HashMap<Uuid, Ticket>>>;

pub async fn create_ticket(
    State(store): State<TicketStore>,
    Json(payload): Json<CreateTicket>,
) -> (StatusCode, Json<Ticket>) {
    let ticket = Ticket {
        id: Uuid::new_v4(),
        title: payload.title,
        description: payload.description,
        status: TicketStatus::Todo,
    };

    store.write().await.insert(ticket.id, ticket.clone());

    (StatusCode::CREATED, Json(ticket))
}

pub async fn get_ticket(
    State(store): State<TicketStore>,
    Path(id): Path<Uuid>,
) -> Result<Json<Ticket>, StatusCode> {
    let tickets = store.read().await;

    tickets
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn patch_ticket(
    State(store): State<TicketStore>,
    Path(id): Path<Uuid>,
    Json(payload): Json<PatchTicket>,
) -> Result<Json<Ticket>, StatusCode> {
    let mut tickets = store.write().await;

    let ticket = tickets.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;

    if let Some(title) = payload.title {
        ticket.title = title;
    }

    if let Some(description) = payload.description {
        ticket.description = description;
    }

    if let Some(status) = payload.status {
        ticket.status = status;
    }

    Ok(Json(ticket.clone()))
}

#[tokio::main]
pub async fn main() {
    let store: TicketStore = Arc::new(RwLock::new(HashMap::new()));

    let app = Router::new()
        .route("/tickets", post(create_ticket))
        .route("/tickets/{id}", get(get_ticket).patch(patch_ticket))
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind TCP listener");

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("Server failed");
}
