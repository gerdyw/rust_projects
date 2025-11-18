pub mod board_cell;
pub mod board_direction;
pub mod coordinate;
pub mod grid;
pub mod move_direction;

pub mod player_board;
pub mod puzzle;
pub mod word;
pub use board_cell::BoardCell;
pub use board_direction::BoardDirection;
pub use coordinate::Coordinate;
pub use grid::Grid;
pub use move_direction::MoveDirection;
pub use player_board::PlayerBoard;
pub use puzzle::Puzzle;
pub use word::Word;
// pub mod board;
// pub mod common;
// pub mod message;
// pub mod puzzle;
// pub mod view;

// use puzzle::Puzzle;
// use ratatui::{
//     DefaultTerminal,
//     crossterm::event::{self, Event, KeyCode, KeyEventKind},
// };

// use crate::model::{
//     board::{Board, Tile},
//     common::{BoardDirection, MoveDirection},
//     message::Message,
// };

// #[derive(Clone, PartialEq, Eq, Debug)]
// pub enum RunningState {
//     Running,
//     Quit,
// }

// pub struct Model {
//     pub puzzle: Puzzle,
//     pub player_board: Board,
//     pub running_state: RunningState,
// }

// impl Model {
//     pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
//         let puzzle = Puzzle::from_file(path)?;
//         let player_board = Board::from_puzzle(&puzzle);
//         Ok(Model {
//             puzzle,
//             player_board,
//             running_state: RunningState::Running,
//         })
//     }

//     pub fn run(mut self, mut terminal: DefaultTerminal) {
//         terminal
//             .draw(|frame| {
//                 self.draw(frame);
//             })
//             .unwrap();

//         while self.running_state == RunningState::Running {
//             let key_opt = self.retrieve_key_pressed();
//             let Some(key) = key_opt else {
//                 continue;
//             };

//             let (new_model, message_opt) = self.handle_key(key);
//             self = new_model;

//             // Process any follow-up message once
//             if let Some(message) = message_opt {
//                 let (updated_model, _) = self.update(message);
//                 self = updated_model;
//             }

//             terminal
//                 .draw(|frame| {
//                     self.draw(frame);
//                 })
//                 .unwrap();
//         }
//     }

//     fn handle_key(self, key: KeyCode) -> (Self, Option<Message>) {
//         match key {
//             KeyCode::Char(c) if c.is_ascii_alphabetic() => self.handle_enter_char(c),
//             KeyCode::Backspace => self.handle_delete_char(),
//             KeyCode::Left => self.handle_move_cursor(MoveDirection::Left),
//             KeyCode::Right => self.handle_move_cursor(MoveDirection::Right),
//             KeyCode::Up => self.handle_move_cursor(MoveDirection::Up),
//             KeyCode::Down => self.handle_move_cursor(MoveDirection::Down),
//             KeyCode::Tab => self.handle_swap_direction(),
//             KeyCode::Esc => (
//                 Model {
//                     running_state: RunningState::Quit,
//                     ..self
//                 },
//                 None,
//             ),
//             _ => (self, None),
//         }
//     }

//     pub fn update(self, message: Message) -> (Self, Option<Message>) {
//         match message {
//             Message::MoveInDirection(dir) => self.handle_move_cursor(dir),
//             Message::MoveForward => self.handle_move_forward(),
//             Message::SwapDirection => self.handle_swap_direction(),
//             Message::MoveToNextWord => self.handle_move_to_next_word(),
//             Message::EnterChar(c) => self.handle_enter_char(c),
//             Message::DeleteChar => self.handle_delete_char(),
//             Message::ResetPuzzle => (
//                 Model {
//                     player_board: Board::from_puzzle(&self.puzzle),
//                     ..self
//                 },
//                 None,
//             ),
//             Message::Quit => (
//                 Model {
//                     running_state: RunningState::Quit,
//                     ..self
//                 },
//                 None,
//             ),
//         }
//     }

//     fn retrieve_key_pressed(&self) -> Option<KeyCode> {
//         // Implementation for retrieving key events and converting them to Messages
//         let key = match event::read().ok()? {
//             Event::Key(key_event) if key_event.kind == KeyEventKind::Press => Some(key_event.code),
//             _ => None,
//         };

//         key
//     }

//     fn handle_move_cursor(self, dir: MoveDirection) -> (Self, Option<Message>) {
//         (
//             Model {
//                 player_board: self.player_board.move_cursor(dir),
//                 ..self
//             },
//             None,
//         )
//     }

//     fn handle_move_forward(self) -> (Self, Option<Message>) {
//         (
//             Model {
//                 player_board: self.player_board.move_forward(),
//                 ..self
//             },
//             None,
//         )
//     }

//     fn handle_move_to_next_word(self) -> (Self, Option<Message>) {
//         (
//             Model {
//                 player_board: self.player_board.move_to_next_word_start(),
//                 ..self
//             },
//             None,
//         )
//     }

//     fn handle_swap_direction(self) -> (Self, Option<Message>) {
//         (
//             Model {
//                 player_board: self.player_board.swap_direction(),
//                 ..self
//             },
//             None,
//         )
//     }

//     fn handle_enter_char(self, c: char) -> (Self, Option<Message>) {
//         let board_after_entry = self.player_board.enter_char(c);
//         if board_after_entry.is_end_of_word() {
//             (
//                 Model {
//                     player_board: board_after_entry,
//                     ..self
//                 },
//                 Some(Message::MoveToNextWord),
//             )
//         } else {
//             (
//                 Model {
//                     player_board: board_after_entry,
//                     ..self
//                 },
//                 Some(Message::MoveForward),
//             )
//         }
//     }

//     fn handle_delete_char(self) -> (Self, Option<Message>) {
//         if self.player_board.grid.get(self.player_board.pos) == &Tile::Empty {
//             // If current cell is empty, move backward and delete that cell
//             let board = self.player_board.move_backward();
//             (
//                 Model {
//                     player_board: board.delete_char(),
//                     ..self
//                 },
//                 None, // Don't move again - we already moved backward
//             )
//         } else {
//             // If current cell is filled, delete it and move backward
//             let direction = self.player_board.direction;
//             (
//                 Model {
//                     player_board: self.player_board.delete_char(),
//                     ..self
//                 },
//                 Some(Message::MoveInDirection(match direction {
//                     BoardDirection::Across => MoveDirection::Left,
//                     BoardDirection::Down => MoveDirection::Up,
//                 })),
//             )
//         }
//     }
// }

// #[cfg(test)]
// #[path = "model/model_tests.rs"]
// mod model_tests;
