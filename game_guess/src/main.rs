use std::io; //including the io library

//new function with no arg
fn main() {
    println!("Welcome to the guessing game, have fun!");//macro: with !
    println!("Now please input your guessed number:");
    
    //mutable, an instance of a String, empty instance
    let mut guessed_num = String::new();

    //call the stdin function form the io module
    io::stdin().read_line(&mut guessed_num) // append at the end without overwriting. &:reference, no copy. &mut: mutable ref
    .expect("Failed to read the input number...");// error handling: Result value - Err and Ok. Result value returned from read_line()
   
    println!("Your guessed number is {guessed_num}");

    // then we need a random number
    //add rand crate in Cargo.toml
    use rand::Rng;//import random - Rng trait
    
}
