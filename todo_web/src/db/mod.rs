use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
pub mod metadata;

pub async fn init_db(db_url: &String) -> Pool<Postgres> {
    println!("Connecting to PostgreSQL with URL: {}", db_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("Failed to create PostgreSQL connection pool");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    pool
}
