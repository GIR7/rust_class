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

/*

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