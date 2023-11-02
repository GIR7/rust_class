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
        let max_x = self.state.iter().map(|&(x, _)| x).max().unwrap_or(0);
        let max_y = self.state.iter().map(|&(_, y)| y).max().unwrap_or(0);

        for y in (0..=max_y).rev() {
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
}