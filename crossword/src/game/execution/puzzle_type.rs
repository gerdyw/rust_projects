pub enum PuzzleType {
    Ipuz,
    Toml,
}
impl PuzzleType {
    pub fn from_file_path(path: &String) -> Option<Self> {
        if path.ends_with(".ipuz") {
            Some(PuzzleType::Ipuz)
        } else if path.ends_with(".toml") {
            Some(PuzzleType::Toml)
        } else {
            None
        }
    }
}
