#[cfg(test)]
mod tests {
    use crate::model::common::{BoardDirection, Coordinate, Grid, MoveDirection};

    #[test]
    fn test_board_direction_swap() {
        assert_eq!(BoardDirection::Across.swap(), BoardDirection::Down);
        assert_eq!(BoardDirection::Down.swap(), BoardDirection::Across);
    }

    #[test]
    fn test_coordinate_move_right() {
        let coord = Coordinate::new(0, 0, 5);
        let moved = coord.move_right();
        assert_eq!(moved.col, 1);
        assert_eq!(moved.row, 0);
    }

    #[test]
    fn test_coordinate_move_right_wrap() {
        let coord = Coordinate::new(4, 0, 5);
        let moved = coord.move_right();
        assert_eq!(moved.col, 0);
        assert_eq!(moved.row, 1);
    }

    #[test]
    fn test_coordinate_move_down() {
        let coord = Coordinate::new(0, 0, 5);
        let moved = coord.move_down();
        assert_eq!(moved.col, 0);
        assert_eq!(moved.row, 1);
    }

    #[test]
    fn test_coordinate_move_down_wrap() {
        let coord = Coordinate::new(0, 4, 5);
        let moved = coord.move_down();
        assert_eq!(moved.col, 1);
        assert_eq!(moved.row, 0);
    }

    #[test]
    fn test_coordinate_move_left() {
        let coord = Coordinate::new(1, 0, 5);
        let moved = coord.move_left();
        assert_eq!(moved.col, 0);
        assert_eq!(moved.row, 0);
    }

    #[test]
    fn test_coordinate_move_left_wrap() {
        let coord = Coordinate::new(0, 1, 5);
        let moved = coord.move_left();
        assert_eq!(moved.col, 4);
        assert_eq!(moved.row, 0);
    }

    #[test]
    fn test_coordinate_move_up() {
        let coord = Coordinate::new(0, 1, 5);
        let moved = coord.move_up();
        assert_eq!(moved.col, 0);
        assert_eq!(moved.row, 0);
    }

    #[test]
    fn test_coordinate_move_up_wrap() {
        let coord = Coordinate::new(1, 0, 5);
        let moved = coord.move_up();
        assert_eq!(moved.col, 0);
        assert_eq!(moved.row, 4);
    }

    #[test]
    fn test_coordinate_move_direction() {
        let coord = Coordinate::new(2, 2, 5);

        let up = coord.move_direction(MoveDirection::Up);
        assert_eq!((up.col, up.row), (2, 1));

        let down = coord.move_direction(MoveDirection::Down);
        assert_eq!((down.col, down.row), (2, 3));

        let left = coord.move_direction(MoveDirection::Left);
        assert_eq!((left.col, left.row), (1, 2));

        let right = coord.move_direction(MoveDirection::Right);
        assert_eq!((right.col, right.row), (3, 2));
    }

    #[test]
    fn test_grid_from_vec() {
        let vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::from_vec(vec);
        assert_eq!(grid.vec().len(), 3);
    }

    #[test]
    #[should_panic(expected = "Invalid grid")]
    fn test_grid_invalid() {
        let vec = vec![
            vec![1, 2, 3],
            vec![4, 5], // Wrong length
        ];
        Grid::from_vec(vec);
    }

    #[test]
    fn test_grid_get() {
        let vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::from_vec(vec);
        let coord = Coordinate::new(1, 1, 3);
        assert_eq!(*grid.get(coord), 5);
    }

    #[test]
    fn test_grid_set() {
        let vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::from_vec(vec);
        let coord = Coordinate::new(1, 1, 3);
        let new_grid = grid.set(coord, 42);
        assert_eq!(*new_grid.get(coord), 42);
    }

    #[test]
    fn test_grid_set_immutable() {
        let vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::from_vec(vec);
        let coord = Coordinate::new(1, 1, 3);
        let _new_grid = grid.clone().set(coord, 42);
        // Original grid should be unchanged
        assert_eq!(*grid.get(coord), 5);
    }

    #[test]
    fn test_grid_iterator() {
        let vec = vec![vec![1, 2], vec![3, 4]];
        let grid = Grid::from_vec(vec);
        let values: Vec<i32> = grid.grid_iter().copied().collect();
        assert_eq!(values, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_grid_row_iterator() {
        let vec = vec![vec![1, 2], vec![3, 4]];
        let grid = Grid::from_vec(vec);
        let rows: Vec<&Vec<i32>> = grid.row_iter().collect();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], &vec![1, 2]);
        assert_eq!(rows[1], &vec![3, 4]);
    }
}
