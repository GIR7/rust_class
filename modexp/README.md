# Modular Exponentiation in Rust

**Author:** Yihui Tian

## Project Overview

This project is a Rust implementation of modular exponentiation. The main goal of this project is to calculate `(x^y) % m` efficiently and accurately for given inputs `x`, `y`, and `m` from the command line.

## How It Works

The core functionality of this project is provided by the `modexp` function, which is defined in the code. This function takes three arguments: `x`, `y`, and `m`, and it returns `(x^y) % m` as a `u64` result. If command-line arguments aren't valid, then `error()` and `parsenum()` would print error message and exit the program.

## Test

Run test case using `cargo test` 

Run `cargo check` and `cargo clippy`, warrnings free.

Run the program with negative x and y and non-positive m, program prints out he error message and aborted
