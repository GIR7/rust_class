use chomp_game::Board;

use prompted::input;

fn main() {
    //input! macro returns a String, 
    //convert it to usize.
    let width:usize = input!("input a numer size width: ").parse::<usize>().unwrap();
    let height:usize = input!("input a numer size width: ").parse::<usize>().unwrap();
    //creates a new game board
    let mut game_board = Board::new(width,height);
    //display the game board
    game_board.print();
    
   
}
