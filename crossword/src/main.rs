use crossword::execution::{CommandExecutor, KeyParser, Runner};
use crossword::model::{PlayerBoard, Puzzle};

fn main() -> std::io::Result<()> {
    let puzzle = Puzzle::from_file("puzzles/puzzle.toml").expect("Failed to load puzzle");
    let board = PlayerBoard::from_puzzle(puzzle);

    let parser = KeyParser::new();
    let executor = CommandExecutor::new(board);
    let mut runner = Runner::new(parser, executor);

    runner.run()?;

    Ok(())
}
