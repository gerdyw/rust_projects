use std::{fs, path::Path};

use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};

pub async fn init_db(db_url: &String) -> Pool<Sqlite> {
    let db_path = Path::new(db_url);

    // Ensure the database file exists
    if !db_path.exists() {
        fs::File::create(db_path).expect("Failed to create SQLite database file");
    }

    let connection_string = format!("sqlite://{}", db_url);
    println!("connection_string: {}", connection_string);

    let pool = SqlitePoolOptions::new()
        .connect(&connection_string)
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    pool
}
