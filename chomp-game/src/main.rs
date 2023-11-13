use chomp_game::Board;

use prompted::input;

fn main() {
    //input! macro returns a String,
    //convert it to usize?
    let width: usize = input!("input a number for width of the board: ")
        .parse()
        .unwrap();
    let height: usize = input!("input a number for height of the board: ")
        .parse()
        .unwrap();
    //creates a new game board
    let mut game_board = Board::new(width, height);
    //display the game board
    println!("Initial board:");
    game_board.print();

    loop {
        // Input the coordinates for the chomp move from the user
        let chomp_x: usize = input!("Input the x-coordinate for the squares you want to chomp: ")
            .parse()
            .unwrap();
        let chomp_y: usize = input!("Input the y-coordinate for the squares you want to chomp: ")
            .parse()
            .unwrap();

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
                continue;
            }
        }

        //check the winning status after player's move
        if game_board.check_winning() {
            println!("You Win!");
            break;
        }

        // AI's turn
        if let Some(ai_winning_move) = game_board.winning_move() {
            println!("AI's move: {:?}", ai_winning_move);
            //pass in the AI's move to chomp the square
            game_board
                .chomp_squares(ai_winning_move.0, ai_winning_move.1)
                .unwrap();
            println!("Board after AI's winning move:");
            game_board.print();

            // Check if the AI wins
            if game_board.check_winning() {
                println!("AI Wins!");
                break;
            }
        } else {
            // No winning move found, perform a stall move
            if let Some(stall_move) = game_board.stall_move() {
                // Execute the stall move by the AI
                println!("AI performs a stall move: {:?}", stall_move);
                game_board
                    .chomp_squares(stall_move.0, stall_move.1)
                    .unwrap();
                println!("Board after AI's stall move:");
                game_board.print();
            } else {
                // No stall move found, board is empty
                println!("Empty board, You lost!");
                break;
            }
        }
    }
}
