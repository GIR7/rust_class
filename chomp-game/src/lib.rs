use std::collections::HashSet;

///Board struct
/// state field: using hashset to store the state(true-not yet eaten) of single element's index
pub struct Board{
    state:HashSet<(usize,usize)>,
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
        }
    }

    ///prints the board 
    //- Ref: chatgpt
    pub fn print(&self) {
        //calculates the maximum x coordinate in the HashSet:state
        let max_x = self.state.iter().map(|&(x, _)| x).max().unwrap_or(0);//if empty then return 0
        //then gets the maximum y in the hashset
        let max_y = self.state.iter().map(|&(_, y)| y).max().unwrap_or(0);

        //loop throught the hashset and print out the state
        for y in (0..=max_y).rev() {//print it from the top of the grid
            for x in 0..=max_x {
                if self.state.contains(&(x, y)) {
                    print!("O ");
                } else {
                    print!("X ");
                }
            }
            println!();
        }
    }

    /// This function checks the board status and to determine if the player or AI already won or lose. 
    /// This function will be called after every move made by AI or the player
    /// If the status of the game is already won or lose, then it will return the results of the game 
    /// 
}