use toy_rsa_lib::*;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent.
pub fn genkey() -> (u32, u32) {
    loop {
        let p = rsa_prime(); //returns a u32
        let q = rsa_prime();
        let p = u64::from(p); //converts to u64 for lcm function
        let q = u64::from(q);
        let lambda = lcm(p - 1, q - 1);
        if EXP < lambda && gcd(EXP, lambda) == 1 {
            return (p.try_into().unwrap(), q.try_into().unwrap()); // function returns a (u32,u32)
        }
    }
}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
/// key: pass in the p*q
/// msg: plaintext
pub fn encrypt(key: u64, msg: u32) -> u64 {
    //type convert
    let msg: u64 = msg as u64;
    let exp: u64 = EXP;
    let key: u64 = key;

    //m^e mod n, where n = p*q
    modexp(msg, exp, key)
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
/// msg: ciphertext
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let (p, q) = key;
    let p = u64::from(p); //converts to u64 for lcm function
    let q = u64::from(q);

    //Calculate 𝜆(n)  = lcm(p-1,q-1)
    let lambda = lcm(p - 1, q - 1);

    // Calculate d as the modular inverse of E mod 𝜆(n)
    let d = modinverse(EXP, lambda);

    //for ciphertext msg, msg^d mod(n) where n = p * q
    let decrypted_msg = modexp(msg, d, u64::from(p) * u64::from(q)); //returns a u64

    decrypted_msg as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    ///hardcoded individual function testing - using test case in canvas
    fn functions_test() {
        let key: (u32, u32) = (0xed23e6cd, 0xf050a04d);
        let msg = 0x12345f;
        assert_eq!(
            0x6418280e0c4d7675,
            encrypt(u64::from(key.0) * u64::from(key.1), msg)
        );
        assert_eq!(msg, decrypt(key, 0x6418280e0c4d7675));
    }

    #[test]
    fn test_toy_rsa() {
        let key = genkey();
        let plaintext: u32 = 12345;
        // Encrypt the message
        let ciphertext = encrypt(u64::from(key.0) * u64::from(key.1), plaintext);
        // Check decrypted message matches the plaintext
        assert_eq!(plaintext, decrypt(key, ciphertext));
    }

    #[test]
    fn test_toy_rsa_zero() {
        let key = genkey();
        let plaintext: u32 = 0;
        // Encrypt the message
        let ciphertext = encrypt(u64::from(key.0) * u64::from(key.1), plaintext);
        // Check decrypted message matches the plaintext
        assert_eq!(plaintext, decrypt(key, ciphertext));
    }
}
