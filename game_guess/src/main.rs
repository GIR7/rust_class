use std::io; //including the io library

//new function with no arg
fn main() {
    println!("Welcome to the guessing game, have fun!");//macro: with !
    
    // then we need a random number
    //add rand crate in Cargo.toml
    use rand::Rng;//import
    let secret_num = rand::thread_rng()//seeded by OS
                    .gen_range(1..=100);//number generated within the range
    println!("The secret number is {}", secret_num);


    loop{
        println!("Now please input your guessed number:");

        //mutable, an instance of a String, empty instance
        let mut guessed_num = String::new();
        //call the stdin function form the io module
        io::stdin().read_line(&mut guessed_num) // append at the end without overwriting. &:reference, no copy. &mut: mutable ref
        .expect("Failed to read the input number...");// error handling: Result value - Err and Ok. Result value returned from read_line()
    
        println!("Your guessed number is {guessed_num}");

        //we need add a new line below for comparison
        //var reuse - when you want to convert a value from one type to another type.
        let guessed_num:u32 = guessed_num.trim()//get rid of whitespace in a string
                                        .parse()//type convert: in this case - u32
                                        .expect("Please type a number");// result handling
        //now camparing the guessed number and secret number
        use std::cmp::Ordering;//import- the Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.
        match guessed_num.cmp(&secret_num) // the cmp wil return ex. Ordering::Greater
        {//then match gets the Ordering::Greater value and starts checking each arm’s pattern. 
            Ordering::Less => println!("Too small"),//not match skip, to next line
            Ordering::Greater => println!("Too large!"),//matches, excute the print - match ends
            Ordering::Equal => {println!("Bingo "); break;}
        }
    }


}