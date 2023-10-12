//topic: struct, enum, pattern match, rust genaric 

/* reference counting
*/
use std::cell::RefCell;
 struct Twostring {
        s1: String,
        s2: String,
    }

fn main() {
    /*Rust Generics
Parameters of kind "type"
Most commonly single capital letter (can be camel-case)
Can parameterize a datatype, function or impl (examples/top.rs)
Turbofish supplies explicit type parameters t.update::<u32>(&0); -- complier can't infer what type*/
 
 // Fails to compile bc mistype.
    //let s: u64 = [1u32, 2, 3].iter().cloned().sum();

    //pattern matching
    /*
    Two kinds: "refutable" and "irrefutable"
    'let' requires irrefutable pattern
    'match' allows refutable pattern, but all possible values must be matched
 */
     struct Point {
        x: i64,
        y: i64,
    }

    let mut  p = Point{x: 3, y: 5};
    let z = match p {//Compiler checks that every possible value will be matched, and that later matches are not subsumed by earlier
        Point{x: 3, y: 5} => {
            println!("is good");
            8
        }
        _ => {
            panic!("bad point");
        }
    };
//Rules for move/borrow are the same for pattern variables
//For convenience, you can modify a pattern variable with ref or ref mut to have it borrowed instead of moved
    match p {
        Point {ref mut x, ..} => *x = 3,
        _ => (),
    }
    println!("{}",p.x);


    //enum
    /* 
    pub enum Option<T> {
          None,
          Some(T),
      }
      pub use Option::*;

      // let x: &u8 = 0; // Compiler says no
      let x: Option<&u8> = None;
      // let Some(ptr) = x; // Compiler says no
      match x {
          None => {
              Err("missing x")
          }
          Some(&x) if x == 7 => {
              println!("all good");
              Ok(())
          }
          Some(&x) => {
              Err("bad x")
          }
        }
        */


   // Moving a field out of a struct makes it "partially moved"
    let t = Twostring {
        s1: "hello".to_string(),
        s2: "world".to_string(),
    };
    let v1 = t.s1;
    println!("{}", v1); // prints "hello"
    // println!("{}", t.s1); // compiler error
    // some_function(&t); // compiler error: t partially moved
    // return t; // compiler error: t partially moved


    //RefCell keeps track of the all the borrowd ref at run time
    let r = RefCell::new(0u8);
    // let r1 = r.borrow(); // returns a guard type called 'Ref'
    let  mut rml = r.borrow_mut();
    *rml += 1;
    // drop(rml);
    println!("refcell: {}", rml);
}
