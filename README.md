# Multiple Mutable Borrows in Rust

This repository demonstrates a common error in Rust: attempting to create multiple mutable references to the same variable.  Rust's borrow checker prevents this to avoid data races and ensure memory safety. The example shows how to correctly handle mutable references to avoid this error.

## Bug

The `bug.rs` file contains code that attempts to create multiple mutable references to the same variable, leading to a compile-time error. 

## Solution

The `bugSolution.rs` file provides a corrected version of the code, showing how to correctly handle mutable references to avoid the error.   This typically involves using only a single mutable reference at a time, or using interior mutability techniques.