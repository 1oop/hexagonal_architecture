use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use application::UserService;
use infrastructure::InMemoryUserRepository;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    username: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct UserResponse {
    id: String,
    username: String,
    email: String,
}

struct AppState {
    user_service: UserService,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Dependency Injection
    let user_repo = Arc::new(InMemoryUserRepository::new());
    let user_service = UserService::new(user_repo);
    let state = Arc::new(AppState { user_service });

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/users", post(create_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Json<UserResponse> {
    let user = state
        .user_service
        .create_user(payload.username, payload.email)
        .await
        .unwrap();

    Json(UserResponse {
        id: user.id.to_string(),
        username: user.username,
        email: user.email,
    })
}
