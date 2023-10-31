/*First class functions:
Have their own types
They can be passed as parameters, returned as results, stored in arrays, used as struct and enum fields, etc
They can be created in any block scope
References are available (indeed, a function essentially is a static reference)

 */

 /*Closure


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