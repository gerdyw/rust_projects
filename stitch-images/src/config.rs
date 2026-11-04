use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub temp_images_dir: String,
    pub dest_images_dir: String,
}

static TEMP_IMAGES_VAR: &str = "TEMP_IMAGES_DIR";
static DEST_IMAGES_VAR: &str = "DEST_IMAGES_DIR";
static LEGACY_DEST_IMAGES_VAR: &str = "DEST_IMAGES_VAR";

static DEFAULT_TEMP_IMAGES_DIR: &str = "/data/temp_images";
static DEFAULT_DEST_IMAGES_DIR: &str = "/data/output_images";

impl Config {
    pub fn from_env() -> Self {
        let temp_images_dir = env::var(TEMP_IMAGES_VAR)
            .unwrap_or_else(|_| DEFAULT_TEMP_IMAGES_DIR.to_string());
        let dest_images_dir = env::var(DEST_IMAGES_VAR)
            .or_else(|_| env::var(LEGACY_DEST_IMAGES_VAR))
            .unwrap_or_else(|_| DEFAULT_DEST_IMAGES_DIR.to_string());
        Self { temp_images_dir, dest_images_dir }
    }
}

