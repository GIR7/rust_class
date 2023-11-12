use chomp_game::Board;

use prompted::input;

fn main() {
    //input! macro returns a String, 
    //convert it to usize.
    let width:usize = input!("input a numer size width: ").parse().unwrap();
    let height:usize = input!("input a numer size width: ").parse().unwrap();
    //creates a new game board
    let mut game_board = Board::new(width,height);
    //display the game board
    game_board.print();

    loop{
        // Input the coordinates for the chomp move from the user
        let chomp_x: usize = input!("Input the x-coordinate for chomp squares: ").parse().unwrap();
        let chomp_y: usize = input!("Input the y-coordinate for chomp squares: ").parse().unwrap();

        // Call the chomp_squares function and handle the result
        match game_board.chomp_squares(chomp_x, chomp_y) {
            Ok(()) => {
                // Chomp successful, print the updated board
                println!("Board after chomp based on player's move:");
                game_board.print();
            }
            Err(err) => {
                // Handle the error
                println!("Error: {}", err);
                println!("please try again!");
            }
        }
    }
}
