#[cfg(test)]
mod tests {
    use super::super::{BoardCell, PuzzleCell};

    #[test]
    fn test_is_filled_true() {
        let cell = BoardCell::Filled('A');
        assert!(cell.is_filled());
    }

    #[test]
    fn test_is_filled_false_empty() {
        let cell = BoardCell::Empty;
        assert!(!cell.is_filled());
    }

    #[test]
    fn test_is_filled_false_blocked() {
        let cell = BoardCell::Blocked;
        assert!(!cell.is_filled());
    }

    #[test]
    fn test_is_empty_true() {
        let cell = BoardCell::Empty;
        assert!(cell.is_empty());
    }

    #[test]
    fn test_is_empty_false_filled() {
        let cell = BoardCell::Filled('X');
        assert!(!cell.is_empty());
    }

    #[test]
    fn test_is_empty_false_blocked() {
        let cell = BoardCell::Blocked;
        assert!(!cell.is_empty());
    }

    #[test]
    fn test_is_blocked_true() {
        let cell = BoardCell::Blocked;
        assert!(cell.is_blocked());
    }

    #[test]
    fn test_is_blocked_false_filled() {
        let cell = BoardCell::Filled('Z');
        assert!(!cell.is_blocked());
    }

    #[test]
    fn test_is_blocked_false_empty() {
        let cell = BoardCell::Empty;
        assert!(!cell.is_blocked());
    }

    #[test]
    fn test_from_option_some() {
        let cell = BoardCell::from(Some('T'));
        assert_eq!(cell, BoardCell::Empty);
    }

    #[test]
    fn test_from_option_none() {
        let cell = BoardCell::from(None);
        assert_eq!(cell, BoardCell::Blocked);
    }

    #[test]
    fn test_from_puzzle_cell_fillable() {
        let cell = BoardCell::from(PuzzleCell::Fillable('A'));
        assert_eq!(cell, BoardCell::Empty);
    }

    #[test]
    fn test_from_puzzle_cell_blocked() {
        let cell = BoardCell::from(PuzzleCell::Blocked);
        assert_eq!(cell, BoardCell::Blocked);
    }

    #[test]
    fn test_equality() {
        assert_eq!(BoardCell::Empty, BoardCell::Empty);
        assert_eq!(BoardCell::Blocked, BoardCell::Blocked);
        assert_eq!(BoardCell::Filled('A'), BoardCell::Filled('A'));
        assert_ne!(BoardCell::Filled('A'), BoardCell::Filled('B'));
        assert_ne!(BoardCell::Empty, BoardCell::Blocked);
    }

    #[test]
    fn test_clone() {
        let cell = BoardCell::Filled('X');
        let cloned = cell.clone();
        assert_eq!(cell, cloned);
    }
}
