/*First class functions:
Have their own types
They can be passed as parameters, returned as results, stored in arrays, used as struct and enum fields, etc
They can be created in any block scope
References are available (indeed, a function essentially is a static reference)

 */

 /*Closure
The problem with pure functions is that anything they compute has to be based on arguments
A closure is a function that is allowed to access and mutate its "environment": names that are statically in scope at the point of its declaration
This means that different calls to the function with the same parameters may produce different results
Closures in Rust are mostly first-class, and can access parameters and locals like anything else
Syntax has wacky pipes and stuff; types are optional and inferred
  */

/*Iterator - trait
Produces a sequence of Option<Item> values on calls to its next() method - when next() returns null, its empty

Iterator Adapters
Implementation of Iterator() that operates by consuming the output of some passed iterator; essentially iterators over iterators

FromIterator
Trait used by collect() to make an iterator into a collection
let v:Vec<i32> = vec![1,2,4];
let w:Vec<i32> = v.into_iter().map(|v| v+1).collect(); //- into_iter() consumes data, can't reused after. The iter() won't consume data
Can implement for your collection type to make it collectible 

IntoIterator
Trait for getting a "canonical" iterator for a type
Makes the type work with for loops
    trait IntoIterator where Self::IntoIter::Item == Self::Item {
        type Item;
        type IntoIter: Iterator;
        fn into_iter(self) -> Self::IntoIter;
    }
Every Iterator has a default implementation of IntoIterator, so that you can for loop over an explicit iterator
let v:Vec<i32> = vec![1,2,4];
for c in v{             //v gets consumed, can't used later
    println!("{}",c);
} 
do this instead:
for &c in &v{     //OR - for &c in v.iter(){}
    println!("{}",c);
} 
 */

 /*enumerate
 An iterator that yields the current count and the element during iteration.
  */
fn main() {
   let y = "hello".to_string();

   //closure cannot be moved more than once as it is not `Copy` due to moving the variable `y` out of its environment
   let f = || drop(y); 

   /*use_f(f);
   |          - value moved here
17 |    use_f(f);
   |          ^ value used here after move */
    //    use_f(f);
    //    use_f(f);
}
fn use_f<F:FnOnce()>(_f:F)
{}