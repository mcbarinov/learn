use axum::extract::State;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use axum_extra::response::Html;
use minijinja::Environment;

use base::get_base_jinja_env;

type Templates = Environment<'static>;

pub async fn serve_app() -> Result<(), anyhow::Error> {
    let tpl = init_jinja_env();
    let router = Router::new().route("/p1", get(p1)).with_state(tpl);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, router.into_make_service()).await.unwrap();
    Ok(())
}

async fn p1(State(tpl): State<Templates>) -> impl IntoResponse {
    Html(tpl.get_template("p1.html").unwrap().render(&()).unwrap())
}

fn init_jinja_env() -> Templates {

    let mut env = get_base_jinja_env();
    minijinja_embed::load_templates!(&mut env);
    dbg!(&env);
    env
}
