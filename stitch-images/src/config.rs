use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub temp_images_dir: String,
    pub dest_images_dir: String,
}

static TEMP_IMAGES_VAR: &str = "TEMP_IMAGES_DIR";
static DEST_IMAGES_VAR: &str = "DEST_IMAGES_VAR";

impl Config {
    pub fn from_env() -> Self {
        let temp_images_dir = env::var(TEMP_IMAGES_VAR).expect(&format!("{TEMP_IMAGES_VAR} not set").to_string());
        let dest_images_dir = env::var(DEST_IMAGES_VAR).expect(&format!("{DEST_IMAGES_VAR} not set").to_string());
        Self { temp_images_dir, dest_images_dir }
    }
}

