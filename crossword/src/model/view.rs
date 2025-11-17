use crate::model::{board::Tile, common::Grid};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellView {
    Filled(SelectedState, char),
    Empty(SelectedState),
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectedState {
    TileSelected,
    // RowSelected,
    None,
}

pub struct BoardView<'a> {
    grid: &'a Grid<CellView>,
}

impl Iterator for BoardView<'_> {
    type Item = CellView;

    fn next(&mut self) -> Option<Self::Item> {
        // Implementation goes here
        todo!()
    }
}
