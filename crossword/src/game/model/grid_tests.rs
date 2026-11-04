#[cfg(test)]
mod tests {
    use super::super::{Coordinate, Grid};

    #[test]
    fn test_from_vec() {
        let vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::from_vec(vec, 3, 3);
        assert_eq!(grid.size(), 3);
    }

    #[test]
    #[should_panic(expected = "Grid dimensions")]
    fn test_from_vec_invalid_not_square() {
        let vec = vec![vec![1, 2], vec![3, 4, 5]];
        Grid::from_vec(vec, 2, 2);
    }

    #[test]
    #[should_panic(expected = "Grid dimensions")]
    fn test_from_vec_invalid_wrong_size() {
        let vec = vec![vec![1, 2, 3], vec![4, 5, 6]];
        Grid::from_vec(vec, 3, 3);
    }

    #[test]
    fn test_new() {
        let grid: Grid<i32> = Grid::new(5, 5, 0);
        assert_eq!(grid.size(), 5);

        for row in 0..5 {
            for col in 0..5 {
                let coord = Coordinate::new(col, row);
                assert_eq!(grid.get(coord), Some(0));
            }
        }
    }

    #[test]
    fn test_get_valid() {
        let vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::from_vec(vec, 3, 3);

        assert_eq!(grid.get(Coordinate::new(0, 0)), Some(1));
        assert_eq!(grid.get(Coordinate::new(1, 0)), Some(2));
        assert_eq!(grid.get(Coordinate::new(2, 2)), Some(9));
        assert_eq!(grid.get(Coordinate::new(1, 1)), Some(5));
    }

    #[test]
    fn test_get_out_of_bounds() {
        let vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::from_vec(vec, 3, 3);

        assert_eq!(grid.get(Coordinate::new(-1, 0)), None);
        assert_eq!(grid.get(Coordinate::new(0, -1)), None);
        assert_eq!(grid.get(Coordinate::new(3, 0)), None);
        assert_eq!(grid.get(Coordinate::new(0, 3)), None);
    }

    #[test]
    fn test_iter() {
        let vec = vec![vec![1, 2], vec![3, 4]];
        let grid = Grid::from_vec(vec, 2, 2);

        let items: Vec<(Coordinate, i32)> = grid.iter().collect();
        assert_eq!(items.len(), 4);

        assert_eq!(items[0], (Coordinate::new(0, 0), 1));
        assert_eq!(items[1], (Coordinate::new(1, 0), 2));
        assert_eq!(items[2], (Coordinate::new(0, 1), 3));
        assert_eq!(items[3], (Coordinate::new(1, 1), 4));
    }

    #[test]
    fn test_vec() {
        let vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let grid = Grid::from_vec(vec.clone(), 3, 3);

        assert_eq!(grid.vec(), &vec);
    }
}
