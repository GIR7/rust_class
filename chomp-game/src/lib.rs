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
    //- Ref: chatgpt
    pub fn print(&self) {
        //calculates the existing maximum x coordinate in the HashSet:state
        let max_x = self.state.iter().map(|&(x, _)| x).max().unwrap_or(0);//if empty then return 0
        //then gets the existing maximum y in the hashset
        let max_y = self.state.iter().map(|&(_, y)| y).max().unwrap_or(0);

        // Print the x-axis labels
        print!("   ");
        for x in 0..=max_x {
            print!("{: <3}", x);
        }
        println!();

        // Print the y-axis labels and the board state
        for y in (0..=max_y).rev(){
            print!("{: <3}", max_y - y);
            for x in 0..=max_x {
                if self.state.contains(&(x, max_y - y)) {
                    print!(" O ");
                } else {
                    print!(" X ");
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