#[cfg(test)]
mod tests {
    use super::super::PuzzleCell;

    #[test]
    fn test_is_fillable_true() {
        let cell = PuzzleCell::Fillable('X');
        assert!(cell.is_fillable());
    }

    #[test]
    fn test_is_fillable_false() {
        let cell = PuzzleCell::Blocked;
        assert!(!cell.is_fillable());
    }

    #[test]
    fn test_is_blocked_true() {
        let cell = PuzzleCell::Blocked;
        assert!(cell.is_blocked());
    }

    #[test]
    fn test_is_blocked_false() {
        let cell = PuzzleCell::Fillable('A');
        assert!(!cell.is_blocked());
    }

    #[test]
    fn test_get_char_fillable() {
        let cell = PuzzleCell::Fillable('Z');
        assert_eq!(cell.get_char(), Some('Z'));
    }

    #[test]
    fn test_get_char_blocked() {
        let cell = PuzzleCell::Blocked;
        assert_eq!(cell.get_char(), None);
    }

    #[test]
    fn test_equality_fillable() {
        let cell1 = PuzzleCell::Fillable('A');
        let cell2 = PuzzleCell::Fillable('A');
        let cell3 = PuzzleCell::Fillable('B');

        assert_eq!(cell1, cell2);
        assert_ne!(cell1, cell3);
    }

    #[test]
    fn test_equality_blocked() {
        let cell1 = PuzzleCell::Blocked;
        let cell2 = PuzzleCell::Blocked;

        assert_eq!(cell1, cell2);
    }

    #[test]
    fn test_equality_mixed() {
        let fillable = PuzzleCell::Fillable('X');
        let blocked = PuzzleCell::Blocked;

        assert_ne!(fillable, blocked);
    }

    #[test]
    fn test_clone() {
        let cell = PuzzleCell::Fillable('M');
        let cloned = cell.clone();

        assert_eq!(cell, cloned);
    }

    #[test]
    fn test_various_chars() {
        let chars = vec!['a', 'Z', '1', ' ', '!'];
        for c in chars {
            let cell = PuzzleCell::Fillable(c);
            assert_eq!(cell.get_char(), Some(c));
            assert!(cell.is_fillable());
        }
    }
}
