mod sudoku;
mod input;
mod search;

fn main() {
    // initialising the game before we begin the search
    let mut game = input::take_input();
    game.print();
    println!("\n##################\n");
    search::search(&mut game);
    println!();
    game.set_sol_counter();
    println!("No of solutions = {}",game.get_no_of_solutions() );
}
