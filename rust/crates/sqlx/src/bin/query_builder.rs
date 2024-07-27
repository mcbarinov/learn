use sqlx::{Executor, FromRow, PgPool, Postgres, QueryBuilder};
use sqlx::postgres::PgPoolOptions;

const CREATE_TABLE: &str = r#"
create table if not exists data (
    id int primary key,
    a text,
    b text,
    c text
);
"#;

#[derive(Debug, FromRow)]
struct Data {
    id: i32,
    a: Option<String>,
    b: Option<String>,
    c: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new().max_connections(5).connect("postgres://localhost:5432/sqlx").await?;
    pool.execute("drop table if exists data;").await?;
    pool.execute(CREATE_TABLE).await?;

    sqlx::query("insert into data (id) values (1)").execute(&pool).await?;

    // before
    dbg!(get_data(&pool).await);

    update_data_2(&pool).await;

    // after update
    dbg!(get_data(&pool).await);

    Ok(())
}

async fn update_data_1(pool: &PgPool) {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new("update data set ");
    query.push("a = ");
    query.push_bind("new value");
    query.push(" where id = ").push_bind(1);

    query.build().execute(pool).await.unwrap();
}

async fn update_data_2(pool: &PgPool) {
    let mut query: QueryBuilder<Postgres> = QueryBuilder::with_arguments("update data set a = $1", vec!["new value"]);
    query.push("a = ");
    query.push_bind("new value");
    query.push(" where id = ").push_bind(1);

    query.build().execute(pool).await.unwrap();
}

async fn get_data(pool: &PgPool) -> Data {
    sqlx::query_as("select * from data where id = 1").fetch_one(pool).await.expect("Failed to fetch data")
}
