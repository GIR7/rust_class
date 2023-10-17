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
 */