//topic: struct, enum, pattern match, rust genaric 

/* reference counting
*/
use std::cell::RefCell;
 struct Twostring {
        s1: String,
        s2: String,
    }

fn main() {
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
