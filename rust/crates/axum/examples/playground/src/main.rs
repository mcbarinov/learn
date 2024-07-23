use axum::extract::State;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

#[derive(Clone)]
struct InnerState {}

#[derive(Clone)]
struct OuterState {}

async fn inner_handler(state: State<InnerState>) {}

async fn outer_handler(state: State<OuterState>) {}

#[tokio::main]
async fn main() {
    let inner_router = Router::new()
        .route("/bar", get(inner_handler))
        .with_state(InnerState {});

    let app = Router::new()
        .route("/", get(outer_handler))
        .nest("/foo", inner_router)
        .with_state(OuterState {});

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
