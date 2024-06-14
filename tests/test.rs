extern crate sudokount_new;
use sudokount_new::sudoku::Sudoku;

#[cfg(test)]
mod tests {
    use std::vec;

    use sudokount_new::sudoku;

    use super::*;

    #[test]
    fn test_set_and_get_field() {
        let mut sudoku = Sudoku::new(8); // Assuming Sudoku::new initializes a 64x64 board
        sudoku.set_field(0, 0, 5);
        assert_eq!(sudoku.get_field(0, 0), 1 << (5 - 1));
    }

    #[test]
    fn test_unset_field() {
        let mut sudoku = Sudoku::new(8);
        sudoku.set_field(0, 0, 5);
        sudoku.unset_field(0, 0, 5);
        assert_eq!(sudoku.get_field(0, 0), 0);
    }

    #[test]
    fn test_get_num_possibilities() {
        let mut sudoku = Sudoku::new(8);
        sudoku.set_field(0, 0, 5);
        sudoku.set_field(0, 0, 3);
        assert_eq!(sudoku.get_num_possibilities(0, 0), 2);
    }

    #[test]
    fn test_get_single_remaining_value() {
        let mut sudoku = Sudoku::new(8);
        sudoku.set_field(0, 0, 5);
        assert_eq!(sudoku.get_single_remaining_value(0, 0), 5);
    }

    #[test]
    fn test_is_digit_present() {
        let mut sudoku = Sudoku::new(8);
        sudoku.set_field(0, 0, 5);
        assert!(sudoku.is_digit_present(0, 0, 5));
        assert!(!sudoku.is_digit_present(0, 0, 4));
    }

    #[test]
    fn test_get_possibilities() {
        let mut sudoku = Sudoku::new(8);
        for i in 1..=64 {
            sudoku.set_field(0, 0, i);
        }
        let possibilities = sudoku.get_possibilities(0, 0);
        let test_vec: Vec<usize> = (1..=64).collect();
        assert_eq!(possibilities,test_vec);
    }

    #[test]
    fn test_eliminate() {
        let d = 4;
        let mut sudoku = Sudoku::new(3);
        let mut sudoku_peers_5_5: &Vec<(usize, usize)> = &Vec::new();
        let mut sudoku_peers_5_1: &Vec<(usize, usize)> = &Vec::new();
        {
           sudoku_peers_5_5 = &sudoku.peers.peers[5][5];
        }
        {
           sudoku_peers_5_1 = &sudoku.peers.peers[5][1];
        }

        for &(i,j) in sudoku_peers_5_1.iter() {
            sudoku.set_field(i, j, 3);
            sudoku.set_field(i, j, 4);
            sudoku.set_field(i, j, 5);
        }

        for &(i,j) in sudoku_peers_5_5.iter() {
            sudoku.set_field(i, j, 1);
            sudoku.set_field(i, j, 2);
            sudoku.set_field(i, j, 3);
            sudoku.set_field(i, j, 4);
        }

        sudoku.clear_cell(5, 1);
        sudoku.set_field(5, 1, 3);
        sudoku.set_field(5, 1, 4);


        sudoku.clear_cell(5, 5);
        sudoku.set_field(5, 5, d);
        sudoku.print();
        println!("\n");
        sudoku.eliminate(5, 5, d);
        sudoku.print();
        // assert_eq!(sudoku.get_field(0, 0), 1 << (3 - 1));
        for &(i,j) in sudoku_peers_5_5.iter() {
            assert_eq!(sudoku.is_digit_present(i, j, 4), false);
            }
        // assert_eq!(sudoku.get_field(0, 0), 1 << (3 - 1));
        for &(i,j) in sudoku_peers_5_1.iter() {
            assert_eq!(sudoku.is_digit_present(i, j, 3), false);
            }
    }


}