use std::{error::Error, fs, slice::Iter};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    debug_log::debug_log,
    game::model::{BoardDirection, Coordinate, Grid, Puzzle, PuzzleCell, Word, puzzle::PuzzleWords},
};

#[derive(Debug, Serialize, Deserialize)]
struct PuzzleFile {
    size: usize,
    grid: String,
    across: Vec<String>,
    down: Vec<String>,
}

pub fn parse_ipuz_to_puzzle(input: &str) -> Result<Puzzle, Box<dyn Error>> {
    let v: Value = serde_json::from_str(input)?;

    // get dimensions
    let width = v
        .get("dimensions")
        .and_then(|d| d.get("width"))
        .and_then(|w| w.as_u64())
        .map(|n| n as usize)
        .or_else(|| v.get("width").and_then(|w| w.as_u64()).map(|n| n as usize))
        .ok_or("missing width in IPUZ")?;

    let height = v
        .get("dimensions")
        .and_then(|d| d.get("height"))
        .and_then(|h| h.as_u64())
        .map(|n| n as usize)
        .or_else(|| v.get("height").and_then(|h| h.as_u64()).map(|n| n as usize));

    // block character (default '#')
    let block_char = v
        .get("block")
        .and_then(|b| b.as_str())
        .and_then(|s| s.chars().next())
        .unwrap_or('#');

    // choose source for letters: prefer "solution", then "puzzle"
    let grid_source = v
        .get("solution")
        .or_else(|| v.get("puzzle"))
        .ok_or("no 'solution' or 'puzzle' array found in IPUZ")?;

    // Build Vec<Vec<PuzzleCell>>
    let mut rows: Vec<Vec<PuzzleCell>> = Vec::new();

    if let Some(arr) = grid_source.as_array() {
        for row_val in arr {
            // Each row might be an array of strings/objects or a string.
            if let Some(row_arr) = row_val.as_array() {
                let mut row_cells = Vec::with_capacity(width);
                for cell_val in row_arr {
                    let cell = if let Some(s) = cell_val.as_str() {
                        // string element like "A" or "#"
                        let ch = s.chars().next().unwrap_or(' ');
                        if ch == block_char {
                            PuzzleCell::Blocked
                        } else {
                            PuzzleCell::Fillable(ch)
                        }
                    } else if let Some(_) = cell_val.as_object() {
                        // object variant: prefer a "solution" string, otherwise treat as blocked
                        if let Some(sol) = cell_val.get("solution").and_then(|x| x.as_str()) {
                            let ch = sol.chars().next().unwrap_or(' ');
                            if ch == block_char {
                                PuzzleCell::Blocked
                            } else {
                                PuzzleCell::Fillable(ch)
                            }
                        } else {
                            // no explicit solution -> treat as blocked (safer than inventing letters)
                            PuzzleCell::Blocked
                        }
                    } else {
                        // unexpected cell format -> blocked
                        PuzzleCell::Blocked
                    };
                    row_cells.push(cell);
                }
                rows.push(row_cells);
            } else if let Some(s) = row_val.as_str() {
                // a single string row, interpret characters and spaces; ignore whitespace
                let mut row_cells = Vec::with_capacity(width);
                for ch in s.chars().filter(|c| !c.is_whitespace()) {
                    if ch == block_char {
                        row_cells.push(PuzzleCell::Blocked);
                    } else {
                        row_cells.push(PuzzleCell::Fillable(ch));
                    }
                }
                // pad/truncate to width
                row_cells.resize_with(width, || PuzzleCell::Blocked);
                rows.push(row_cells);
            } else {
                return Err("unsupported row format in IPUZ".into());
            }
        }
    } else {
        return Err("expected solution/puzzle to be an array of rows".into());
    }

    // Determine actual height from rows if not specified
    let height = height.unwrap_or(rows.len());

    // convert rows into Grid<PuzzleCell>
    let grid = Grid::from_vec(rows, width, height);

    // Extract clues: IPUZ example stores clues under "clues" -> "Across"/"Down" as arrays of [number, text]
    let extract = |dir: &str| -> Vec<String> {
        v.get("clues")
            .and_then(|c| c.get(dir))
            .and_then(|a| a.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|item| {
                        if let Some(s) = item
                            .as_array()
                            .and_then(|a| a.get(1))
                            .and_then(|x| x.as_str())
                        {
                            s.to_string()
                        } else if let Some(s) = item.as_str() {
                            s.to_string()
                        } else {
                            item.to_string()
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    };

    let across = extract("Across");
    let down = extract("Down");

    // Build words & clue numbers using existing helper
    let all_words = find_words(&grid, across.clone(), down.clone());

    let mut across_words = Vec::new();
    let mut down_words = Vec::new();
    let mut clue_numbers = Vec::new();

    for w in all_words {
        clue_numbers.push((w.start_pos, w.clue_number));
        match w.direction {
            BoardDirection::Across => across_words.push(w),
            BoardDirection::Down => down_words.push(w),
        }
    }

    clue_numbers.sort_by(|a, b| a.1.cmp(&b.1));
    clue_numbers.dedup_by(|a, b| a.1 == b.1);

    Ok(Puzzle::new(
        grid,
        PuzzleWords {
            across: across_words,
            down: down_words,
        },
        clue_numbers,
    ))
}

pub fn from_toml_file(path: &str) -> Result<Puzzle, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    from_toml_string(contents)
}

pub fn from_toml_string(puzzle_data: String) -> Result<Puzzle, Box<dyn Error>> {
    let puzzle_file: PuzzleFile = toml::from_str(&puzzle_data)?;
    let mut grid_vec = Vec::new();

    for line in puzzle_file.grid.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            let row: Vec<PuzzleCell> = trimmed
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|ch| match ch {
                    '.' => PuzzleCell::Blocked,
                    c => PuzzleCell::Fillable(c),
                })
                .collect();
            grid_vec.push(row);
        }
    }

    let height = grid_vec.len();
    let width = grid_vec.get(0).map(|r| r.len()).unwrap_or(puzzle_file.size);

    let grid = Grid::from_vec(grid_vec, width, height);
    let words = find_words(&grid, puzzle_file.across, puzzle_file.down);

    let mut across = Vec::new();
    let mut down = Vec::new();
    let mut clue_numbers = Vec::new();
    let mut all_words = Vec::new();

    for word in words {
        clue_numbers.push((word.start_pos, word.clue_number));

        all_words.push(word.clone());

        match word.direction {
            BoardDirection::Across => across.push(word),
            BoardDirection::Down => down.push(word),
        }
    }

    clue_numbers.sort_by(|a, b| a.1.cmp(&b.1));
    clue_numbers.dedup_by(|a, b| a.1 == b.1);

    Ok(Puzzle::new(
        grid,
        PuzzleWords { across, down },
        clue_numbers,
    ))
}

fn find_words(
    grid: &Grid<PuzzleCell>,
    across_clues: Vec<String>,
    down_clues: Vec<String>,
) -> Vec<Word> {
    let mut words = Vec::new();
    let mut across_iter = across_clues.iter();
    let mut down_iter = down_clues.iter();
    let mut prev_clue_number = 0;

    for (start_pos, _) in grid.iter() {
        let clue_number = prev_clue_number + 1;
        if let Some(across_word) = find_word(
            grid,
            &mut across_iter,
            start_pos,
            clue_number,
            BoardDirection::Across,
        ) {
            words.push(across_word);
            prev_clue_number = clue_number;
        }

        if let Some(down_word) = find_word(
            grid,
            &mut down_iter,
            start_pos,
            clue_number,
            BoardDirection::Down,
        ) {
            words.push(down_word);
            prev_clue_number = clue_number;
        }
    }

    for word in &words {
        debug_log(format!(
            "{} word # {} \"{}\" at ({}-{})",
            word.direction, word.clue_number, word.text, word.start_pos, word.end_pos
        ));
    }
    words
}

fn is_across_word_start(grid: &Grid<PuzzleCell>, coord: Coordinate) -> bool {
    let left = coord.left();
    let right = coord.right();
    let get = |c| grid.get(c).unwrap_or(PuzzleCell::Blocked);

    get(coord).is_fillable() && get(left).is_blocked() && get(right).is_fillable()
}

fn is_down_word_start(grid: &Grid<PuzzleCell>, coord: &Coordinate) -> bool {
    let coord = *coord;
    let up = coord.up();
    let down = coord.down();
    // Helper to get char from grid
    let get = |c: Coordinate| {
        grid.get(c).and_then(|cell| match cell {
            PuzzleCell::Fillable(ch) => Some(ch),
            PuzzleCell::Blocked => None,
        })
    };

    // Current cell is fillable, up is blocked/edge, down is fillable
    get(coord).is_some() && get(up).is_none() && get(down).is_some()
}

fn is_word_start(grid: &Grid<PuzzleCell>, coord: Coordinate, direction: BoardDirection) -> bool {
    match direction {
        BoardDirection::Across => is_across_word_start(grid, coord),
        BoardDirection::Down => is_down_word_start(grid, &coord),
    }
}

fn find_word(
    grid: &Grid<PuzzleCell>,
    clue_iter: &mut Iter<'_, String>,
    start_pos: Coordinate,
    clue_number: usize,
    direction: BoardDirection,
) -> Option<Word> {
    if !is_word_start(grid, start_pos, direction) {
        return None;
    }

    let clue = clue_iter.next().map(|s| s.to_owned());

    let Some(clue) = clue else {
        debug_log(format!(
            "No clue found for word starting at {:?} in direction {:?}",
            start_pos, direction
        ));
        return None;
    };

    let (cells, text) = grid
        .directional_iter(start_pos, direction.into(), false)
        .take_while(|(coord, cell)| cell.is_fillable() && coord.on_same_line(start_pos, direction))
        .fold(
            (Vec::new(), String::new()),
            |(mut vec, mut s), (coord, cell)| {
                vec.push((coord, cell));
                if let PuzzleCell::Fillable(ch) = cell {
                    s.push(ch);
                } else {
                    panic!("Blocked cell encountered in word");
                }
                (vec, s)
            },
        );

    let coords = cells
        .iter()
        .map(|(coord, _)| *coord)
        .collect::<Vec<Coordinate>>();

    let end_pos = start_pos.add_to_axis(direction, text.len() as isize - 1);

    Some(Word::new(
        text,
        clue_number,
        direction,
        start_pos,
        end_pos,
        clue,
        coords,
    ))
}
