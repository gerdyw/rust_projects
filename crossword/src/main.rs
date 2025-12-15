use crossword::debug_log::debug_log;
use crossword::execution::{CommandExecutor, KeyParser, PuzzleType, Runner};
use crossword::model::{PlayerBoard, Puzzle};

fn main() -> std::io::Result<()> {
    // Parse the puzzle file path from command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <puzzle-file-path>", args[0]);
        eprintln!("Example: {} puzzles/BB-8.ipuz", args[0]);
        std::process::exit(1);
    }

    let puzzle_path = &args[1];
    let puzzle_type = PuzzleType::from_file_path(puzzle_path).expect("puzzle type unsupported");

    // Read the puzzle file into a String
    let puzzle_content = std::fs::read_to_string(puzzle_path)?;

    let puzzle = match puzzle_type {
        PuzzleType::Ipuz => {
            Puzzle::parse_ipuz_to_puzzle(&puzzle_content).expect("Failed to parse IPUZ puzzle")
        }
        PuzzleType::Toml => {
            Puzzle::from_toml_file(&puzzle_path).expect("Failed to parse TOML puzzle")
        }
    };

    let board = PlayerBoard::from_puzzle(puzzle);
    let parser = KeyParser::new();
    let executor = CommandExecutor::new(board);
    let mut runner = Runner::new(parser, executor);

    runner.run()?;

    Ok(())
}
