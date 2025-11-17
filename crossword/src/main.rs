use crossword::model::Model;

fn main() {
    // Example usage
    let model = Model::from_file("puzzles/puzzle.toml").expect("Failed to load puzzle");
    let mut terminal = ratatui::init();
    model.run(terminal);
    ratatui::restore();
}
