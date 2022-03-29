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

## Types

has two main catagories of types

- scaler types represent a single value
- compound types represent a group of values

## Scaler Types

### Integers

rust has signed and unsigned integers

| size    | signed | unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |

rust also allows you to use the cpu architectures native size with `isize` and `usize`

```rs
let x: i8 = 5; // 8 bit signed
let y: u128 = 5; // 128 bit unsigned
let z: isize = 5; // architecture dependant unsigned
```

### Floats

integers are available in 32-bit and 64-bit

```rs
let x: f32 = 1.0; // 32 bit float
let x: f64 = 1.0; // 64 but float
```

### Boolean

```rs
let x: bool = true;
```

### Characters

in rust characters unicode and take 4 bytes

```rs
let x: char = 'a';
let y = '😀';
```

## Compound Types

### Arrays

arrays are fixed length

```rs
let x: [i32; 5] = [1, 2, 3, 4, 5];
```

the `5` after the `;` is the size of the array

arrays can be indexed with `[]`

```rs
let mut arr = [3, 2, 1];
arr[2] = 4;
let x = arr[0];
```

### Tuples

tuples are like arrays but each position can be a diffrent type

```rs
let tup: (i32; f32; bool) = (5, 1.0, false);
```

tuples cannot be dynamically indexed but you can access values with `.`

```rs
let mut tup = (5, 1.0, false);
tup.1 = 3.14;
let x: bool = tup.2;
```

tuples can also be destructured like in JavaScript and Python

```rs
let tup = (3.2, 17, 2);
let (a, b, c) = tup;
let sum = b + c;
```

### Type conversion

type conversion is not done for you. You have to use the `as` key word

```rs
let my_int: i32 = 5;
let my_float: f32 = my_int as f32;
```

## Functions

use `fn` to make a function

```rs
fn main() {
    say_hello();
}

fn say_hello() {
    println!("hello");
}
```

every rust program has a `main` function

you can pass values to functions with arguments

specify a return type using `->`

you can return with `return`

```rs
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

or by putting an expression on the last line

```rs
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Expressions and Statements

in Rust expressions return values and statements do not

examples of statements include

```rs
let mut x = 5;
x = 1;
fn add_one(x: i32) -> i32 {x + 1}
add_one(3);
println!("hello world");
```

anything that ends in `;` is a statement

examples of expressions include

```rs
5
x
x + 5
add_one(3)
```

you can create expressions using `{}`

for example

```rs
{
    println!("hello world");
    let x = 3;
    x + 2
}
```

is a n expression that prints "hello world" creates a variable called `x` with value 3 and then evaluates to 3 + 2

so we can do

```rs
let a = {
    println!("hello world");
    let x = 3;
    x + 2
};
```

which will set print "hello world" and set a = 5

note that the last line is an expression and doesn't have a `;` just like in a function that returns something

## Comments

you can add a comment with `//`

```rs
// a comment on it's own line
let x = 5; // a comment at the end of a line
```
