use std::sync::Arc;

use axum::{extract::State, Router, routing::get};
use axum::response::IntoResponse;
use tokio::net::TcpListener;

struct LibraryState {
    s1: String,
}

struct AppState {
    s2: String,
}

async fn library_handler(state: State<Arc<LibraryState>>) -> impl IntoResponse {
    state.s1.clone()
}
async fn app_handler(state: State<Arc<AppState>>) -> impl IntoResponse {
    state.s2.clone()
}

#[tokio::main]
async fn main() {
    let library_state = LibraryState { s1: "library".to_string() };
    let library_state = Arc::new(library_state);

    let app_state = AppState { s2: "app".to_string() };
    let app_state = Arc::new(app_state);

    let library_router = get_library_router::<Arc<AppState>>(library_state);

    let app = Router::new().route("/", get(app_handler)).nest("/library", library_router).with_state(app_state);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// T is a OuterState
fn get_library_router<T>(state: Arc<LibraryState>) -> Router<T> {
    Router::new().route("/", get(library_handler)).with_state(state)
}
