pub struct Settings {
    pub database_url: String,
    pub port: u16,
}

pub fn load_settings() -> Settings {
    Settings {
        database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        port: std::env::var("PORT")
            .unwrap_or("3000".to_string())
            .parse()
            .unwrap(),
    }
}
