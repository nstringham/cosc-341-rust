# Rust

- Compiled low level general purpose language
- designed as an alternative for C and C++
- Main advantage over C is memory safety
  - this means no null pointer exceptions
- created by Graydon Hoare at Mozilla in 2010

## Hello World

```rs
fn main() {
    println!("Hello, world!");
}
```

## Installing Rust

Install from [rust-lang.org](https://www.rust-lang.org/tools/install)

Includes:

- `rustc` the rust compiler
- `rustup` a tool for updating rust
- `cargo` rust's official package manager

## Variables

here is how to make a variable

```rs
let x: i32 = 5;
```

or

```rs
let x = 5;
```

type annotations are optional if they can be inferred

variables are immutable my default (like `final` in Java)

to make them mutable add `mut`

```rs
let mut x = 5;
x = 6;
```

Rust also has constants which are evaluated at compile time (like `#define` in C/C++)

```rs
const X: i32 = 5;
```

types are not optional for constants

