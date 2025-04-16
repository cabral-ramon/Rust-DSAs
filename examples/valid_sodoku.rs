use std::collections::{HashMap, HashSet};

fn main() {}

fn is_valid_sodoku(board: Vec<Vec<char>>) -> bool {
    let mut rows: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut columns: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut squares: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

    for r in 0..board.len() {
        for c in 0..board.len() {
            let item = board[r][c];
            if item == '.' {
                continue;
            }

            let square_coord = (r / 3, c / 3);

            let row = rows.entry(r).or_default();
            let col = columns.entry(c).or_default();
            let square = squares.entry(square_coord).or_default();

            if row.contains(&item) || col.contains(&item) || square.contains(&item) {
                return false;
            }

            row.insert(item);
            col.insert(item);
            square.insert(item);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sudoku_hashmap() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sodoku(board), true);
    }

        #[test]
    fn test_invalid_sudoku_hashmap() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(is_valid_sodoku(board), false);
    }
}
