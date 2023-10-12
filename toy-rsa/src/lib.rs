use toy_rsa_lib::* ;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;


/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent.
pub fn genkey() -> (u32, u32){
    loop{
        let p = rsa_prime();//returns a u32
        let q = rsa_prime();
        let p = u64::from(p);//converts to u64 for lcm function
        let q = u64::from(q);
        let lambda = lcm(p-1,q-1);
        if EXP < lambda && gcd(EXP,lambda)==1{
            return (p.try_into().unwrap(),q.try_into().unwrap());// function returns a (u32,u32)
        }
    }
}


/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64{
    let x: u64 = msg as u64;
    let y: u64 = EXP;
    let m: u64 = key;

    modexp(x, y, m)
}


/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) {
    
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_function() {
        let p: u64 = 7;
        let q: u64 = 11;
        // Expected result
        let expected_result: u64 = 30;  // LCM of (7 - 1) and (11 - 1) = LCM(6, 10) = 30

        // Call the help function
        let result = helper(p, q);

        // Assert that the result matches the expected value
        assert_eq!(result, expected_result);
    }
}
