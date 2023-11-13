use std::collections::HashSet;

///Board struct
/// state field: using hashset to store the state(true-not yet eaten) of single element's index
/// width and height, represents the size of the board
#[derive(Clone)]
pub struct Board {
    state: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Board {
    ///Generates a new game board, size based on the user's input
    pub fn new(width: usize, height: usize) -> Self {
        let mut game_state = HashSet::new();
        for i in 0..width {
            for j in 0..height {
                game_state.insert((i, j));
            }
        }

        Board {
            state: game_state,
            width,
            height,
        }
    }

    ///Print a graphical representation of a board.
    pub fn print(&self) {
        // Print the x axis labels
        print!("    ");
        for x in 0..self.width {
            print!("{: <3}", x);
        }
        println!();

        for y in 0..self.height {
            //Print the y axis labels
            print!("{: <3}", y);
            //Print representation
            for x in 0..self.width {
                if self.state.contains(&(x, y)) {
                    print!(" X ");
                } else {
                    print!(" O ");
                }
            }
            println!();
        }
    }

    /// Chomp a given square,
    /// removing all squares below it and to the right of it.
    /// Panic if the input is invalid
    pub fn chomp_squares(&mut self, x: usize, y: usize) -> Result<(), &'static str> {
        // Make sure the input is within the board size
        if x < self.width && y < self.height {
            // Then make sure the square can still be chomped
            if self.state.contains(&(x, y)) {
                // Remove all squares
                for i in x..self.width {
                    for j in y..self.height {
                        self.state.remove(&(i, j));
                    }
                }
                Ok(())
            } else {
                Err("Square has already been chomped")
            }
        } else {
            Err("Invalid coordinates")
        }
    }

    pub fn check_winning(&self) -> bool {
        //if the board only have the one top left square
        self.state.len() == 1 && self.state.contains(&(0, 0))
    }

    ///winning_move function, returns a winning move if there is any
    pub fn winning_move(&self) -> Option<(usize, usize)> {
        // Check if the board state is already lost
        if self.state.len() == 1 && self.state.contains(&(0, 0)) {
            return None; // No winning move
        } else {
            //for each possible move m:

            // Iterate through each remaining row and column
            for r in 0..self.height {
                for c in 0..self.width {
                    if r == 0 && c == 0 {
                        continue;
                    }

                    // Create a new board p
                    let mut new_board = self.clone();
                    // Perform the move m on p
                    if new_board.chomp_squares(c, r).is_ok() {
                        // Call winning_move recursively on the new board p
                        if new_board.winning_move().is_none() {
                            // No winning move found for the opponent, so (c,r) is a winning move
                            return Some((c, r));
                        }
                    }
                }
            }
        }
        // No winning move found
        None
    }

    /// Chomps the furthest-right piece in the lowermost nonempty row.
    pub fn stall_move(&self) -> Option<(usize, usize)> {
        // Find the lowermost nonempty row
        if let Some(lowest_row) = self.state.iter().map(|&(_, y)| y).max() {
            // Find the rightmost square in the lowermost nonempty row that hasn't been chomped
            if let Some(rightmost_square) = (0..self.width)
                .rev()
                .find(|&x| self.state.contains(&(x, lowest_row)))
            {
                return Some((rightmost_square, lowest_row));
            }
        }
        // No stall move found
        None
    }
}

#[test]
fn test_create_board() {
    let width = 3;
    let height = 4;
    let board = Board::new(width, height);

    assert_eq!(board.width, width);
    assert_eq!(board.height, height);
}

#[test]
fn test_chomp_squares() {
    let mut board = Board::new(3, 3);

    assert!(board.chomp_squares(2, 2).is_ok());
    assert_eq!(board.state.len(), 8);
}

#[test]
fn test_check_winning() {
    let mut board = Board::new(3, 3);

    assert!(!board.check_winning());

    // Chomp all squares except the top-left one
    for x in (0..3).rev() {
        for y in (0..3).rev() {
            // Skip (0, 0)
            if x == 0 && y == 0 {
                continue;
            }
            assert!(board.chomp_squares(x, y).is_ok());
        }
    }
    assert_eq!(board.check_winning(), true);
}

#[test]
fn test_stall_move() {
    let mut board = Board::new(3, 3);

    // Chomp some squares to create a stall situation
    for x in (1..3).rev() {
        assert!(board.chomp_squares(x, 0).is_ok());
    }

    assert_eq!(board.stall_move(), Some((0, 2)));
}
