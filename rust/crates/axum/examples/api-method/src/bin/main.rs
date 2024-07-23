use axum::{Json, Router};
use axum::body::Body;
use axum::extract::{Host, Path, Request};
use axum::http::{HeaderName, HeaderValue};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use reqwest::{Client, Method};
use serde_json::json;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/data/:id/handler1", post(handler1))
        .route("/api/data/:id/handler2", delete(handler2))
        .route("/api-post/*path", get(api_method))
        .route("/api-delete/*path", get(api_method));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler1(Path(id): Path<String>) -> impl IntoResponse {
    format!("handler1: {}", id)
}

async fn handler2(Path(id): Path<String>) -> impl IntoResponse {
    Json(json!({"response" : id}))
}

async fn api_method(Host(hostname): Host, req: Request) -> Response<Body> {
    let mut uri = req.uri().to_string();
    let method = if uri.starts_with("/api-post/") {
        uri = uri.replacen("/api-post/", "/api/", 1);
        Method::POST
    } else if uri.starts_with("/api-delete/") {
        uri = uri.replacen("/api-delete/", "/api/", 1);
        Method::DELETE
    } else {
        panic!("unsupported api method: {}", uri);
    };
    let url = format!("http://{hostname}{uri}");
    let res = Client::new().request(method, url).send().await.unwrap();
    let headers = res.headers().clone();

    let mut response: Response<Body> = Response::builder().body(Body::from_stream(res.bytes_stream())).unwrap();

    response.headers_mut().extend(headers.into_iter().map(|(name, value)| {
        let name = HeaderName::from_bytes(name.unwrap().as_ref()).unwrap();
        let value = HeaderValue::from_bytes(value.as_ref()).unwrap();
        (name, value)
    }));
    response
}
