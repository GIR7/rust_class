//topic: struct, enum, pattern match, rust genaric 

/* reference counting
*/
use std::cell::RefCell;


fn main() {

   // Moving a field out of a struct makes it "partially moved"
    let t = TwoString {
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
