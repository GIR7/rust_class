# TOY RSA Encryption Library in Rust

**Author:** Yihui Tian

## Project Overview

This is a simple RSA encryption library implemented in Rust. It provides functions for generating RSA key pairs, encrypting and decrypting messages using RSA.

### Note:
This library relies on an external crate, `toy_rsa_lib`, to provide certain essential functions for RSA encryption.

## How It Works

### Key Generation

The `genkey` function generates a pair of prime numbers (`p` and `q`) within a specified range suitable for RSA encryption with a given exponent (`EXP`).

### Encryption

The `encrypt` function takes the public `key` and a plaintext message `msg`.

### Decryption

The `decrypt` function takes the private key (a tuple containing `p` and `q`) and the ciphertext.


## Test

Run test case using `cargo test` 

    Test cases includes the individual function test, the overall functionality test.

Run `cargo check` and `cargo clippy`, warrnings free.