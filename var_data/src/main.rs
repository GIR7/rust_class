fn main() {
    let x = 5;// x=6 - error: cannot assign twice to immutable variable `x`
    
    let mut y = 6;//mutable 
    y = 7; 

    //Constants 
    //- can't using mut
    //- type of value must be annotated
    //- can be declared in any scope
    //- be set only to a constant expression, not the result of a value that could only be computed at runtime

    //name conventio for constant: all uppercase with underscores
    const TIME_MIN:u32 = 60*3;


    //Shadowing-shadow a variable by using the same variable’s name and repeating the use of the let keyword
    let x = 5;
    let x = x + 1;//creates a new var

    {
        let x = x * 2;//creates a new var
        println!("The value of x in the inner scope is: {x}"); // 6
    }
    println!("The value of x is: {x}");// 12


    //Data Types - Rust is statically typed 
    //scalar: int, float, booleans, chars
   /*Integer
   Int without a fractional component 
    - u32(unsigned integer with 32 bit space, signed-i32)
    when number is unsigned, it is positive
    _ as a visual separator 1000 => 1_000
    defualt type: i32
    */

    // Float - All floating-point types are signed.
    let xf = 2.0; // f64
    let yf: f32 = 3.0; // f32

    //Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    //Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    //The Character Type - four bytes in size and represents a Unicode Scalar Value, 
    //which means it can represent a lot more than just ASCII. 
    //Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


    //compound: tuples and arrays
    //tuples - create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}, ");//6.4 
    let s = tup.0;
    println!("{s}");//500

    //array - the number of elements will not need to change.
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; //i32 type, 5 elements

    //when using semicolon
    //b will contain 5 elements that will all be set to the value 3
    let b = [3; 5];//same as b = a = [3, 3, 3, 3, 3];

    //indexing arr
    let aa = [1, 2, 3, 4, 5];
    let first = aa[0];
    let second = aa[1];





}
