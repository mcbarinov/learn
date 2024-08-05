use std::collections::HashMap;
use std::sync::Arc;

use axum::{Form, middleware, Router};
use axum::body::Body;
use axum::extract::{Query, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::get;
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use cookie::time::Duration;
use minijinja::{context, Environment, path_loader};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    access_token: String,
    template: Environment<'static>,
}

#[tokio::main]
async fn main() {
    let state = AppState { template: init_templates(), access_token: "my-secret".to_string() };
    let state = Arc::new(state);
    let app = Router::new()
        .route("/", get(index_page))
        .route("/login", get(login_page).post(login))
        .route("/logout", get(logout))
        .layer(middleware::from_fn_with_state(Arc::clone(&state), access_token_middleware))
        .with_state(state);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Templates

type HtmlResponse = Result<Html<String>, AppError>;

fn init_templates() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));
    env
}

fn render_template<S: Serialize>(template_name: &str, context: S, env: &Environment) -> Result<Html<String>, AppError> {
    let tmpl = env.get_template(template_name)?;
    let content = tmpl.render(context)?;
    Ok(Html(content))
}

async fn index_page(State(state): State<Arc<AppState>>) -> HtmlResponse {
    render_template("index.html", context! {}, &state.template)
}

#[derive(Serialize, Deserialize)]
struct LoginForm {
    access_token: String,
}

async fn login_page(State(state): State<Arc<AppState>>) -> HtmlResponse {
    render_template("login.html", context! {}, &state.template)
}

async fn login(State(state): State<Arc<AppState>>, mut jars: CookieJar, Form(form): Form<LoginForm>) -> (CookieJar, Redirect) {
    if form.access_token == state.access_token.clone() {
        let cookie =
            Cookie::build(("access-token", state.access_token.clone())).path("/").http_only(true).max_age(Duration::days(30));
        jars = jars.add(cookie);
    }
    (jars, Redirect::to("/"))
}

async fn logout(mut jars: CookieJar) -> (CookieJar, Redirect) {
    let cookie = Cookie::build(("access-token", "")).path("/").http_only(true);
    jars = jars.add(cookie);
    (jars, Redirect::to("/"))
}

// AppError via anyhow

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Something went wrong: {}", self.0)).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// access token middleware
async fn access_token_middleware(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Query(query_params): Query<HashMap<String, String>>,
    request: Request,
    next: Next,
) -> Response {
    let mut auth_ok = false;

    if request.uri() != "/login" {
        // query
        if let Some(value) = query_params.get("access-token") {
            auth_ok = value == &state.access_token;
        }

        // header
        if let Some(value) = request.headers().get("access-token") {
            if let Ok(value) = value.to_str() {
                auth_ok = value == state.access_token;
            }
        }

        // cookie
        if let Some(value) = jar.get("access-token") {
            auth_ok = value.value() == state.access_token;
        }
    } else {
        auth_ok = true;
    }
    if auth_ok {
        next.run(request).await
    } else {
        Response::builder().status(StatusCode::UNAUTHORIZED).body(Body::from("access denied")).unwrap()
    }
}
