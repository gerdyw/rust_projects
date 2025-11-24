use crossword::execution::{CommandExecutor, KeyParser, Runner};
use crossword::model::{PlayerBoard, Puzzle};

fn main() -> std::io::Result<()> {
    // read the puzzles/Arent_We_a_Pair.ipuz file into a String
    let ipuz_puzzle = std::fs::read_to_string("puzzles/BB-8.ipuz")?;
    let puzzle = Puzzle::parse_ipuz_to_puzzle(&ipuz_puzzle).expect("Failed to parse IPUZ puzzle");
    let board = PlayerBoard::from_puzzle(puzzle);

    let parser = KeyParser::new();
    let executor = CommandExecutor::new(board);
    let mut runner = Runner::new(parser, executor);

    runner.run()?;

    Ok(())
}
