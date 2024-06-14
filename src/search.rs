use std::sync::atomic::Ordering;
use crate::sudoku::{Sudoku, SOLUTION_COUNTER}; // Replace with the actual path to your Sudoku struct and SOLUTION_COUNTER

pub fn search(sudoku: &mut Sudoku) {
    match sudoku.eliminate_all(){
      Ok(changed) => {
          // println!("Elimination successfull, changed: {}", changed);
      },
      Err(err) => {
          println!("Error: {}", err);
      }
    };

    // eliminate all also populates the mrv field of the game
    // println!("MRV: {:?} and value is {}", sudoku.get_mrv(), sudoku.get_field(sudoku.get_mrv().0, sudoku.get_mrv().1));
    // now we can copy the game in the search function and start the search 
    let mut stack: Vec<Sudoku> = vec![sudoku.clone()];

    while stack.len() > 0 {
        // println!("Stack size = {}", stack.len());
        let mut game: Sudoku = stack.pop().unwrap();
        match game.delete_twins() {
            Ok(changed) => {
                // println!("Naked twins successfull, changed: {}", changed);
            },
            Err(err) => {
                // println!("Error: {}", err);
            }
        }
        game.min_rem_val();
        let (x, y) = game.get_mrv();
        // println!("MRV = {},{}", x, y);
        if (x,y) == (usize::MAX, usize::MAX) { // No more MRV
            SOLUTION_COUNTER.fetch_add(1, Ordering::SeqCst);
            println!("Solution found, total solutions = {}", SOLUTION_COUNTER.load(Ordering::SeqCst));
            // println!("Got to solution:");
            // game.print();
            continue;
        }
        let possibilities = game.get_possibilities(x, y);
        // println!("Possibilities len = {}", possibilities.len());

        if possibilities.len() == 1 {
            println!("$$$$ No possibilities found at {},{}", x, y);
            continue;
        }
        for possibility in possibilities{
          let mut puzzle = game.clone();
          puzzle.clear_cell(x, y);
          puzzle.set_field(x, y, possibility);
          match puzzle.eliminate(x, y, possibility){
            Ok(changed) => {
              // println!("Elimination successfull at {},{} value={}, changed: {}", x, y, possibility, changed);
              if changed {
                // puzzle.print();
                stack.push(puzzle.clone());
              }
                // puzzle.print();
            },
            Err(err) => {
              // println!("Error: {}", err);
              continue;
            }
          } 
        }
    }
    
}