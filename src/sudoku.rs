use std::sync::atomic::{AtomicUsize, Ordering};
pub static SOLUTION_COUNTER: AtomicUsize = AtomicUsize::new(0);
use std::collections::BinaryHeap;
use std::cmp::Ordering as OrdOrdering;

#[derive(Eq, PartialEq, Clone)]
struct CellPossibilities{
  num_possibilities: usize,
  coordinates: (usize, usize),
}

// Implement Ord so that cells with fewer possibilities are given higher priority
impl Ord for CellPossibilities {
  fn cmp(&self, other: &Self) -> OrdOrdering {
      // Note that we flip the order here for min-heap behavior
      other.num_possibilities.cmp(&self.num_possibilities)
  }
}

impl PartialOrd for CellPossibilities {
  fn partial_cmp(&self, other: &Self) -> Option<OrdOrdering> {
      Some(self.cmp(other))
  }
}

pub struct SudokuPeers{
  size: usize,
  len: usize,
  pub peers: Vec<Vec<Vec<(usize, usize)>>>,
}

impl SudokuPeers {
  // build peer is builds a set of all the cells that are in the same row, column, or box as the cell at (x, y)
  // it populates the peers field of the Sudoku struct
  pub fn new(size: usize) -> Self {
      let len = size * size;
      // let sqrt = (size as f64).sqrt() as usize;
      let mut peers = vec![vec![Vec::new(); len]; len];

      for i in 0..len {
          for j in 0..len {
              // Add cells in the same row
              for x in 0..len {
                  if x != i {
                      peers[i][j].push((x, j));
                  }
              }
              // Add cells in the same column
              for y in 0..len {
                  if y != j {
                      peers[i][j].push((i, y));
                  }
              }
              // Add cells in the same box
              let box_start_row = i - i % size;
              let box_start_col = j - j % size;
              for x in box_start_row..box_start_row + size {
                  for y in box_start_col..box_start_col + size {
                      // if x != i || y != j {
                      // if (x,y) != (i,j) && (x!=i || y!=j) {
                      if x!=i && y!=j {
                          peers[i][j].push((x, y));
                      }
                  }
              }
          }
      }

      SudokuPeers { size, len, peers }
  }
}



#[derive(Clone)]
pub struct Sudoku{
  size: usize,
  len: usize,
  pub peers: &'static SudokuPeers,
  mrv: (usize,usize),
  board: Vec<Vec<u64>>,
  no_solution: u64,
  // cell_possibilities: BinaryHeap<CellPossibilities>,
}


impl Sudoku {

  pub fn new(size: usize) -> Sudoku {
    let len = size * size;
    let board = vec![vec![0; len]; len];
    let peers = Box::leak(Box::new(SudokuPeers::new(size)));
    let mrv: (usize, usize) = (0,0);
    let no_solution = 0;
    Sudoku { size, len, peers, mrv,  board, no_solution }
  }

  pub fn set_sol_counter(&mut self) {
    self.no_solution = SOLUTION_COUNTER.load(Ordering::SeqCst) as u64;
  }
  
  pub fn get_mrv(&self) -> (usize, usize) {
    self.mrv
  }

  pub fn get_peers(&self, i: usize, j: usize) -> &Vec<(usize, usize)> {
    &self.peers.peers[i][j]
  }

  pub fn get_no_of_solutions(&self) -> u64 {
    self.no_solution
  }

  pub fn clear_cell(&mut self, x: usize, y: usize) {
    self.board[x][y] = 0;
  }

  pub fn print_peers(&self) {
    for (x, row) in self.peers.peers.iter().enumerate() {
        for (y, peers) in row.iter().enumerate() {
            print!("Peers for cell ({}, {}): ", x, y);
            for (x, y) in peers {
                print!("({}, {}), ", x, y);
            }
            println!();
        }
    }
  }

  pub fn set_field(&mut self, x: usize, y: usize, value: usize) {
    if value == 0 {
      println!("Value must be between 1 and {}", self.len);
      return;
    }
    self.board[x][y] |= 1 << (value - 1);
  }

  pub fn unset_field(&mut self, x: usize, y: usize, value: usize) {
    self.board[x][y] &= !(1 << (value - 1));
  }

  pub fn get_field(&self, x: usize, y: usize) -> u64 {
    self.board[x][y]
  }

  pub fn print(&self) {
    for x in 0..self.len {
        for y in 0..self.len {
            print!("|");
            let possibilities = self.get_possibilities(x, y);
            let pos_len = possibilities.len();
            for possibility in possibilities {
                print!("{},", possibility);
            }
            if pos_len > 2 {
              print!("|\t"); 
            }else{
              print!("|\t\t");
            }
        }
        println!();
    }
}
  
  pub fn print_raw(&self) {
    for row in self.board.iter() {
      for col in row.iter() {
        print!("{:016x} ", col);
      }
      println!();
    }
  }

  pub fn get_num_possibilities(&self, x: usize, y: usize) -> usize {
    self.board[x][y].count_ones() as usize
  }

  pub fn get_single_remaining_value(&self, x: usize, y: usize) -> usize {
    let value = self.get_field(x, y);
    return value.trailing_zeros() as usize + 1;
  }

  pub fn is_digit_present(&self, i:usize, j:usize, d:usize) -> bool {
    let mut value = self.get_field(i, j);
    value & (1 << (d - 1)) != 0
  }

  pub fn get_possibilities(&self, x: usize, y: usize) -> Vec<usize> {
    let mut possibilities = Vec::new();
    let value = self.get_field(x, y) as u64; // Cast to u64 to ensure sufficient size for shifts
    for i in 1..=self.len {
        // Ensure i does not exceed the number of bits in u64 to avoid overflow
        if i > 64 {
            break; // Or handle this case as appropriate for your application
        }
        if (value & (1u64 << (i - 1))) != 0 {
            possibilities.push(i);
        }
    }
    possibilities
}

  pub fn min_rem_val (&mut self){
    let mut min = usize::MAX;
    let mut mrv = (usize::MAX,usize::MAX);
    for i in 0..self.len {
        for j in 0..self.len {
            let num_possibilities = self.get_num_possibilities(i, j);
            if num_possibilities > 1 && num_possibilities < min {
                min = num_possibilities;
                mrv = (i, j);
            }
        }
    }
    self.mrv = mrv;
  }
  // eliminate_all is a function which iterates over all peers of a cell
  // and removes the value of the cell from the peer's possible values
  // iff the value of the cell is already determined
  // returns true if elimination was successful otherwise false
  pub fn eliminate_all(&mut self) -> Result<bool, &'static str> {
    let mut changed = false;
    let mut min_possibilities = usize::MAX;

    for x in 0..self.len {
        for y in 0..self.len {
            let num_possibilities = self.get_num_possibilities(x, y);
            if num_possibilities == 1 {
                let value = self.get_single_remaining_value(x, y);
                let peers_of_curr_cell = self.peers.peers[x][y].iter(); // should consider not cloning
                for (i, j) in peers_of_curr_cell {
                    if self.is_digit_present(*i, *j, value) {
                        self.unset_field(*i, *j, value);
                        if self.get_field(*i, *j) == 0 {
                            return Err("Invalid puzzle: elimination resulted in no possibilities");
                        }
                        changed = true;
                    }
                }
            }
        }
    }
    self.min_rem_val();
    Ok(changed)
  }


  pub fn eliminate(&mut self, x: usize, y: usize, value: usize) -> Result<bool, &'static str> {
    // println!("Called Eliminate {} at {},{}", value, x, y);
    // Check if the cell's value is already determined and matches the given value
    if self.get_num_possibilities(x, y) != 1 || self.get_single_remaining_value(x, y) != value {
        return Ok(false);
    }

    let mut changed = false;
    // Iterate over all peers of the cell
    for &(i, j) in &self.peers.peers[x][y] {
        // Attempt to remove the value from the peer's possibilities
        if self.is_digit_present(i, j, value) {
            self.unset_field(i, j, value);
            changed = true; // Mark that a change was made

            // Check if the peer now has a single remaining possibility
            if self.get_num_possibilities(i, j) == 0 {
                return Err("Invalid puzzle: elimination resulted in no possibilities");
            } else if self.get_num_possibilities(i, j) == 1 {
                let new_value = self.get_single_remaining_value(i, j);
                // Recursively eliminate the new value from the peers of this cell
                self.eliminate(i, j, new_value)?;
            }
        }
    }

    Ok(changed)
  }


  pub fn delete_twins(&mut self) -> Result<bool, &'static str> {
    let mut changed = false;
    for x in 0..self.len {
        for y in 0..self.len {
            let possibilities = self.get_possibilities(x, y);
            if possibilities.len() == 2 {
                let peers = self.peers.peers[x][y].iter();
                for &(i, j) in peers {
                    if self.get_possibilities(i, j) == possibilities {
                        let peer_peers = self.peers.peers[i][j].iter();
                        for &(ii, jj) in peer_peers {
                            if (ii, jj) != (x, y) && (ii, jj) != (i, j) {
                                let mut peer_possibilities = self.get_possibilities(ii, jj);
                                let original = peer_possibilities.clone();
                                peer_possibilities.retain(|&x| !possibilities.contains(&x));
                                if peer_possibilities.is_empty() {
                                    return Err("Invalid puzzle: elimination resulted in no possibilities");
                                }
                                if peer_possibilities != original {
                                  for num in peer_possibilities{
                                    self.set_field(ii, jj, num);
                                    changed = true;
                                  }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(changed)
}
}