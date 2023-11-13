# Chomp Game

**Author:** Yihui Tian

## Overview

Chomp is a player against AI player where players take turns chomping squares from a rectangular grid. The goal is to force your opponent to chomp the last square, which is the top-left square.

This project implements the Chomp game in Rust, providing a simple terminal interface for playing against an AI opponent.

## Features in the crate

- **Board Creation:** Generate a new game board with user's input width and height.
- **Print Board:** Display a graphical representation of the current board in the terminal.
- **Chomp Squares:** Chomp a specified square, removing all squares below it and to the right of it.
- **Winning Move:** The AI opponent uses the negamax algorithm to find a winning move, if one exists.
- **Stall Move:** If there is no winning move, the AI performs a stall move by chomping the furthest-right piece in the lowermost nonempty row.

## Run the program
1. Clone the repository

2. Run the project:

   `
    cargo run
   `

## Test

Run test cases using `cargo test` 

Run `cargo check` and `cargo clippy`, warrnings free.

Passed 4 test cases: test_chomp_squares, test test_check_winning, test_create_board and test test_stall_move 