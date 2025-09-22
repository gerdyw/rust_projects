use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

pub async fn init_db(db_url: &String) -> Pool<Sqlite> {
    let pool = SqlitePoolOptions::new().connect(db_url).await.unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT 0
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}
