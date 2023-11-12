use std::collections::HashSet;

///Board struct
/// state field: using hashset to store the state(true-not yet eaten) of single element's index
pub struct Board{
    state:HashSet<(usize,usize)>,
    width: usize,
    height: usize,
}

impl Board{
    ///Generates a new game board, size based on the user's input
    pub fn new(width:usize,height:usize)->Self{
        let mut game_state = HashSet::new();
        for i in 0..width{
            for j in 0..height{
                game_state.insert((i,j));
            }
        }

        Board{
            state:game_state,
            width,
            height,
        }
    }

    ///Print a graphical representation of a board.
    pub fn print(&self) {
        // Print the x-axis labels
        print!("    ");
        for x in 0..self.width {
            print!("{: <3}", x);
        }
        println!();

       
        for y in (0..self.height) {
            //Print the x-axis labels
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
    pub fn chomp_squares(&mut self,x:usize,y:usize) -> Result <(), &'static str>{
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
    
}