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
pub fn decrypt(key: (u32, u32), msg: u64) ->u32 {
    let (p,q) = key;
    let p = u64::from(p);//converts to u64 for lcm function
    let q = u64::from(q);
    //Calculate 𝜆(p, q) for RSA
    let lambda = lcm(p-1,q-1);
    
    // Calculate d as the modular inverse of E mod 𝜆(p, q)
    let d = modinverse(EXP,lambda );

    //Calculate msg^d mod (p * q)
    let decrypted_msg = modexp(msg, d, u64::from(p) * u64::from(q));//returns a u64

    decrypted_msg as u32
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa_encryption_decryption() {
        // Generate a key pair
        let key = genkey();
        // Original message to be encrypted
        let original_msg: u32 = 12345;
    
        // Encrypt the message
        let encrypted_msg = encrypt(u64::from(key.0)*u64::from(key.1), original_msg);
        // Decrypt the message
        let decrypted_msg = decrypt(key, encrypted_msg);
    
        // Check if the decrypted message matches the original message
        assert_eq!(decrypted_msg, original_msg);
    }
}
