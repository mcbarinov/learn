use sqlx::Executor;
use sqlx::postgres::PgPoolOptions;

const CREATE_TABLE: &str = r#"
create table if not exists data (
    id bigserial primary key,
    name text not null,
    value int not null,
    rate float,
    tags text[] not null,
    created_at timestamptz default now(),
    updated_at timestamptz
);
"#;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new().max_connections(5).connect("postgres://localhost:5432/sqlx").await?;
    pool.execute("drop table if exists data;").await?;
    let r = pool.execute(CREATE_TABLE).await?;
    println!("r: {r:?}");

    // insert a row
    let r = sqlx::query("insert into data (name, value, tags) values ($1, $2, $3)")
        .bind("one")
        .bind(1)
        .bind(&["foo", "bar"])
        .execute(&pool)
        .await?;
    dbg!(r);

    Ok(())
}
