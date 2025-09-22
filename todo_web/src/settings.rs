pub struct Settings {
    pub database_url: String,
}

pub fn load_settings() -> Settings {
    Settings {
        database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    }
}
