//assert panic 
//assert_eq!() // could gets expensive when u put it in the loop - performance cost

/*Implicit Error Types
In std::io we have

  pub type Result<T> = std::result::Result<T, std::io::Error>;
Thus we can use std::io::Result and then say things like

  fn f() -> Result<u32> */


/*Error Handling: Unwrap and Expect
Methods of Option and Result. .unwrap() panics with a generic message on failure, .expect("x") panics with the message "x" on failure

Panic on 'None' or 'Err', otherwise return the contents of 'Some' or 'Ok'

  let n = Some(7).unwrap(); // n is now 7
  let n = None.unwrap(); // program panics
Note .unwrap_or() and unwrap_or_else(). Useful for supplying a default value in a few situations
*/
fn main() {
  /*Can use a Vec as a stack:
  let mut stack = Vec::new();
  let v = stack.pop();
Could return a 'Result' with a custom error indicating popping an empty stack.
Right if intent is for 'pop()' to be a "partial function"
Could return an 'Option' — what 'Vec' does. This indicates that popping an empty stack is not to be regarded as an error, but you should handle it. 
Right if intent is for 'pop()' to be a "total function" */


  // Print a str backward.
  let mut s = "hello".to_string();
  while let Some(c) = s.pop() {
      print!("{}", c);
  }
  println!();

  assert!(is_negative("-100").unwrap());
  assert!(is_negative("-x").is_err());

  //Error Combinators
  let scaled_min = (0..n)         // Create a range from 0 to n
    .min()                      // This method returns an Option<T> containing the minimum value or None if the range is empty.
    .and_then(|s| Some(s / m))  // is used on the Option<T> returned by .min(). and_then takes a closure that applies an operation to the value inside the Some. In this case, it divides s by m and wraps the result in Some, ensuring that scaled_min remains an Option type.
    .or_else(|| Some(0));       // used to provide a default value in case the result of the and_then operation is None. The closure || Some(0) always returns Some(0).
}


//Rust has the '?' operator. This takes an Option or Result and unwraps 
//if good, early-returns from the calling function with the error otherwise
  fn is_negative(s: &str) -> Result<bool, std::num::ParseIntError> {
      let n: i64 = s.parse()?;//if it fails, still returns Results instead of panic like unwrap()
      Ok(n < 0)
  }


//crates - binary crates & library crate
//pub mod dir/filename //-module, also easy to make the sub module
//scope control - name space 

/*Versioning and Semantic Versioning (SemVer):
Crates typically follow the principles of Semantic Versioning (SemVer),
 which includes version numbers like 1.2.3, 
 where the major version may change for breaking changes, 
 minor versions for backwards-compatible additions, 
 and patch versions for backwards-compatible bug fixes. 

 for Rust: 
 Three-part version numbers: breaking.major.minor
Rust convention also says 0.breaking.minor

 Cargo.toml has version matching rules: usually want just the breaking part
Cargo.lock preserves Cargo's versioning choices
 */

 /* By defualt, 
 src/main.rs - binary root
 src/lib.rs - library crate root
 src/mod.rs - module top-level for a library

src/bin/prog.rs is a named binary prog
src/bin/prog/main.rs is a named binary prog with modules
examples/ is a bin directory usually used for example of library crates usage. - 'cargo run --example foo' runs the example named 'foo' from here
tests/ contains extra tests, usually used with library crates
benches/ contains benchmarks [unstable-ish]. Usauslly use 'criterion-rs'
  
  */

  /*Namespaces are :: separated as in some other languages
std::io::stdin()
use std::io::stdin; then stdin();
use std::io::{self, stdin};, then io::stdin() or stdin() 
*/

/*Dependencies in the toml file:
Instead of

  [dependencies]
  foo = { git = "http://github.com/something/foo.git" }
Try

  [dependencies.foo]
  git = "http://github.com/something/foo.git" 
  */

/*'pub'
By default, nothing in a module is visible outside the module/crate in which it is declared

You can put pub in front of an item to make it externally visible

For things like structs and enums, the visibility of the thing and its fields is controlled separately
pub struct foo{f1:int } is different pub struct foo{ pub f1:int } 

The compiler will whine at you about an unused thing unless it is pub, in which case it assumes you just know it will be used somewhere
 */

 /*Docs - rustdoc 
Doc comments have an extra slash ///

Inline doc comments are //!
You can also have C-style doc comments /** or /*!
Use cargo doc --document-private-items to get docs for stuff not visible outside the crate

Doc comments are treated as Markdown
*/

*/Tests
Marking a function with #[test] makes it a test
Tests fail by panicking (#[should_panic] tests fail by not panicking)
Test panics usually from asserts: assert!(), assert_eq!()
Marking an item #[cfg(test)] makes it included only when compiling for testing
Examples in doc comments are treated as "doc tests" by default: must compile and not panic ---- !
  */

//Traits:
/* 
pub trait Vegetable{
  fn desc(&self) -> &'static str;
}

pub struct Lettuce;
impl Vegetable for Lettuce{
  fn desc(&self) -> &'static str{
    self.desc()
  }
}
impl Lettuce{
  fn desc(&self) -> &'static str{
    "Lettuce"
  }
}

pub struct Tomato;
impl Vegetable for Tomato{
  fn desc(&self) -> &'static str{
    self.desc()
  }
}
impl Tomato{
  fn desc(&self) -> &'static str{
    "Tomato"
  }
}

pub struct GenericSalad<T: Vegetable>{
  veggies:Vec<T>, // a 'GenericSalad' is the vector of 'Vegetable' - lettuce, tomato
}
impl <T:Vegetable>GenericSalad<T>{
  fn new() -> Self{
    GenericSalad { veggies: Vec::new() }
  }
  //adds a new 'GenericSalad'
  fn add(&mut self,v:T){
    self.veggies.push(v); // push a veggies to 'GenericSalad'
  }
  //display a 'GenericSalad'
  fn display(&self){
    for v in &self.veggies{
      println!("{}",v.desc()); // display a 'GenericSalad' BY display all the vegetables are in it
    }
  }
}

//dynamic traits:
pub struct TraitSalad{
  veggies: Vec<Box<dyn Vegetable>> // we don't decide it at compile time
}
impl TraitSalad {
  fn new() -> Self{
    TraitSalad { veggies: Vec::new() }
  }
}
*/

/*Generics: 
Hash example: 
Hash<(i32,i32)>, and Has<u64> can't do the comparision like H1 == H2
 */