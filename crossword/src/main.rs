use crossword::model::Puzzle;

fn main() {
    // Example usage
    // let model = Model::from_file("puzzles/puzzle.toml").expect("Failed to load puzzle");
    // let terminal = ratatui::init();
    // model.run(terminal);
    // ratatui::restore();

    let puzzle = Puzzle::from_file("puzzles/puzzle.toml").expect("Failed to load puzzle");
    println!("Loaded puzzle of size {}", puzzle.size());
    println!("{}", puzzle);
}
