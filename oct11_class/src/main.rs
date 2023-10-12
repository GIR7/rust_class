//topic: struct, enum, pattern match, rust genaric 

/* reference counting
*/
use std::cell::RefCell;

fn main() {
    //RefCell keeps track of the all the borrowd ref at run time
    let r = RefCell::new(0u8);
    // let r1 = r.borrow(); // returns a guard type called 'Ref'
    let  mut rml = r.borrow_mut();
    *rml += 1;
    // drop(rml);
    println!("refcell: {}", rml);
}
