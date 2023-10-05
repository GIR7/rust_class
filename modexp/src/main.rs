//! Command-line modular exponentation tool
//! Takes in 3 args throught command line and return the results
//! 
//! Yihui Tian 10/4/2023

use std::process;

fn main() {
   let res:u64 = modexp(10, 9, 6);
   println!("res is {}", res)
}

/// Performs modular exponentiation.
///
/// Given three values x, y, and m, this function calculates (x^y) % m.
/// It returns the result as a u64 value.
fn modexp(x:u64, y:u64, m:u64) -> u64 {
    
    //convert to u128 for calculation:
    let mut x = u128::from(x);
    let mut y = u128::from(y);
    let m = u128::from(m);

    //define return val
    let mut res = 1;

    if m == 0 {
        eprintln!("Invalid input: m is zero.");
        process::exit(1);
    }
    if m == 1 {
        return 0;
    }

    while y > 0{
        if y%2 == 1{
            res = (res*x) % m;
        }
        y >>= 1;// y div 2
        x = (x*x)%m;
    }
    //convert results back to u64
    let res:u64 = u64::try_from(res).unwrap();
    res
}

#[test]
/// Assert testing for function modexp()
  fn test_modexp() {
      // Largest prime less than 2**64.
      // https://primes.utm.edu/lists/2small/0bit.html
      let bigm = u64::max_value() - 58;
      assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
      assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
      assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
      // https://practice.geeksforgeeks.org/problems/
      //    modular-exponentiation-for-large-numbers/0
      assert_eq!(4, modexp(10, 9, 6));
      assert_eq!(34, modexp(450, 768, 517));
  }