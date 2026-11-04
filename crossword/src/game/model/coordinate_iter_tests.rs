#[cfg(test)]
mod tests {
    use super::super::{BoardDirection, Coordinate, CoordinateIter};

    #[test]
    fn test_new() {
        let start = Coordinate::new(0, 0);
        let end = Coordinate::new(3, 0);
        let iter = CoordinateIter::new(start, end);
        assert_eq!(iter.direction(), BoardDirection::Across);
    }

    #[test]
    fn test_direction_across() {
        let iter = CoordinateIter::new(Coordinate::new(0, 5), Coordinate::new(4, 5));
        assert_eq!(iter.direction(), BoardDirection::Across);
    }

    #[test]
    fn test_direction_down() {
        let iter = CoordinateIter::new(Coordinate::new(2, 0), Coordinate::new(2, 4));
        assert_eq!(iter.direction(), BoardDirection::Down);
    }

    #[test]
    fn test_iterate_across() {
        let iter = CoordinateIter::new(Coordinate::new(1, 2), Coordinate::new(4, 2));
        let coords: Vec<Coordinate> = iter.collect();

        assert_eq!(coords.len(), 4);
        assert_eq!(coords[0], Coordinate::new(1, 2));
        assert_eq!(coords[1], Coordinate::new(2, 2));
        assert_eq!(coords[2], Coordinate::new(3, 2));
        assert_eq!(coords[3], Coordinate::new(4, 2));
    }

    #[test]
    fn test_iterate_down() {
        let iter = CoordinateIter::new(Coordinate::new(3, 1), Coordinate::new(3, 4));
        let coords: Vec<Coordinate> = iter.collect();

        assert_eq!(coords.len(), 4);
        assert_eq!(coords[0], Coordinate::new(3, 1));
        assert_eq!(coords[1], Coordinate::new(3, 2));
        assert_eq!(coords[2], Coordinate::new(3, 3));
        assert_eq!(coords[3], Coordinate::new(3, 4));
    }

    #[test]
    fn test_iterate_single_cell() {
        let iter = CoordinateIter::new(Coordinate::new(2, 2), Coordinate::new(2, 2));
        let coords: Vec<Coordinate> = iter.collect();

        assert_eq!(coords.len(), 1);
        assert_eq!(coords[0], Coordinate::new(2, 2));
    }

    #[test]
    fn test_iterate_two_cells_across() {
        let iter = CoordinateIter::new(Coordinate::new(0, 1), Coordinate::new(1, 1));
        let coords: Vec<Coordinate> = iter.collect();

        assert_eq!(coords.len(), 2);
        assert_eq!(coords[0], Coordinate::new(0, 1));
        assert_eq!(coords[1], Coordinate::new(1, 1));
    }

    #[test]
    fn test_iterate_two_cells_down() {
        let iter = CoordinateIter::new(Coordinate::new(5, 0), Coordinate::new(5, 1));
        let coords: Vec<Coordinate> = iter.collect();

        assert_eq!(coords.len(), 2);
        assert_eq!(coords[0], Coordinate::new(5, 0));
        assert_eq!(coords[1], Coordinate::new(5, 1));
    }

    #[test]
    fn test_multiple_iterations() {
        let mut iter = CoordinateIter::new(Coordinate::new(0, 0), Coordinate::new(2, 0));

        assert_eq!(iter.next(), Some(Coordinate::new(0, 0)));
        assert_eq!(iter.next(), Some(Coordinate::new(1, 0)));
        assert_eq!(iter.next(), Some(Coordinate::new(2, 0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None); // Should stay None
    }
}
