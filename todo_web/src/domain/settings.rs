pub struct Settings {
    pub database_location: String,
    pub assets_location: String,
    pub port: u16,
}

pub fn load_settings() -> Settings {
    Settings {
        database_location: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        assets_location: std::env::var("ASSETS_LOCATION").expect("ASSETS_LOCATION must be set"),
        port: std::env::var("PORT")
            .unwrap_or("3000".to_string())
            .parse()
            .unwrap(),
    }
}
