# Rust

-   Compiled low level general purpose language
-   designed as an alternative for C and C++
-   Main advantage over C is memory safety
    -   this means no null pointer exceptions
-   created by Graydon Hoare at Mozilla in 2010

## Hello World

```rs
fn main() {
    println!("Hello, world!");
}
```

## Installing Rust

Install from [rust-lang.org](https://www.rust-lang.org/tools/install)

Includes:

-   `rustc` the rust compiler
-   `rustup` a tool for updating rust
-   `cargo` rust's official package manager

## The Book

Rust has an official book that is a great way to learn rust.
You can read it for free on the rust website https://doc.rust-lang.org/book/

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

-   [scaler types](#scaler-types) represent a single value
-   [compound types](#compound-types) represent a group of values

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

in Rust the empty tuple `()` takes zero bytes and is used as a void

```rs
fn print_message() -> () {
    println!("this function doesn't return anything");
}
```

### Type conversion

type conversion is not done for you. You have to use the `as` key word

```rs
let my_int: i32 = 5;
let my_float: f32 = my_int as f32;
```

## Operators

`+`, `-`, `*`, `/`, `%`, `<`, `>`, `<=`, `>=`, `==`, `!=`, `&&`, `||`, and `!` all work like they do in Java and C/C++

you can also do `+=`, and `-=`

you cannot mix types so you have to convert

```rs
let three: i32 = 3;
let three_fifths: f32 = three as f32 / 5.0
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

## Condition Expressions

Rust has 2 conditional expressions: [`if`](#if) and [`match`](#match)

### If

this code prints a message if x is less than 5

```rs
if x < 5 {
    println!("x is smaller than 5");
}
```

you can also use `else`

```rs
if x < 5 {
    println!("x is smaller than 5");
} else if x == 5 {
    println!("x is equal to 5");
} else {
    println!("x greater than 5");
}
```

because `if` is a expression it evaluates to a value

```rs
let a = 4;
let b = 7;

let bigger = if a > b { a } else { b };
```

this is similar to doing `(a > b) ? a : b` in Java or C/C++

### Match

match statements allow for multiple cases

```rs
let x = 5;
match x {
    0 => {
        println!("x is zero");
    }
    1 => {
        println!("x is one");
    }
    2 => {
        println!("x is two");
    }
    3 | 4 | 5 => {
        println!("x is three, four, or five");
    }
    _ => {
        println!("x is unknown");
    }
}
```

notice that multiple case can be combined with `|`

`_` is the default case if none of the other cases are matched

because `match` is an expression it evaluates to a value

```rs
let color = match x {
    1 => "red",
    2 => "green",
    3 => "blue",
    _ => "yellow",
};
```

## Loops

there are three was to make a loop in Rust [`loop`](#loop), [`while`](#while), and [`for`](#for)

### Loop

this code prints "hello" the the screen over and over forever

```rs
loop {
    println!("hello");
}
```

`continue` skips to the next iteration
`break` to exit a loop

```rs
fn count(n: i32) {
    let i = 0;
    loop {
        if i >= n {
            break;
        }

        println!("{}", i);

        i++;
    }
}
```

`break` can be used with a value

```rs
let number = loop {
    let user_input = my_input_function();

    if user_input < 0> {
        println!("value must be positive");
        continue;
    } else {
        break user_input;
    }
}
```

### While

`while` breaks if a condition is false

```rs
fn count(n: i32) {
    let i = 0;
    while i < n {
        println!("{}", i);

        i++;
    }
}
```

### For

`for` iterates through a list

```rs
let list = [2, 3, 5, 1];

let mut sum = 0;

for element in list {
    sum += element;
}

println!("{}", sum); // outputs 11
```

you can define ranges with `..`

```rs
fn count(n: i32) {
    for i in 0..n {
        println!("{}", i);
    }
}
```

## Comments

you can add a comment with `//`

```rs
// a comment on it's own line
let x = 5; // a comment at the end of a line
```

## Printing to stdout

```rs
println!("hello world");
```

`println` is a macro which is basically like a function

you can use `{}` in insert a variable into the string

```rs
let friends = 5;
let followers = 13;
println!("You have {} friends and {} followers!", friends, followers);
```

you can also put formatting instructions inside the `{}`

```rs
println!("{:0.2}", 1.234);         // prints "1.23"
println!("{:5}", 7);               // prints "    7"
println!("{:<5}", 'x');            // prints "x    "
println!("{:^5}", 'x');            // prints "  x  "
println!("{:>5}", 'x');            // prints "    4"
```

you can use an index or a name or specify what variable goes where

```rs
println!("{1} {0}", 2, 3 );        // prints "3 2"
println!("{b} {a}", a="2", b="3"); // prints "3 2"
```

`print` doesn't add a newline at the end

```rs
print!("how many: ")
io::stdout().flush().unwrap();
```

it also doesn't flush stdout so you will need to do that yourself

## Getting Input from stdin

you can get a line of input by calling read_line on stdin

```rs
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
```

there is no `nextInt` or `scanf` so you will need to parse the string yourself

the `parse` method is generic and is used to parse anything that can be parsed

```rs
let x = input.trim().parse::<i32>().unwrap();
```

thankfully if you don't provide the type it can usually be implied

```rs
let x: i32 = input.trim().parse().unwrap();
```

## The Heap and the Stack

Rust stores variables in 2 places the heep and the stack.

The stack if used for fixed size variables and is planned out at compile time.

The heep is used for memory that must be allocated at runtime because it has an unknown size at compile time.

All of the scaler and compound types have a size known at compile time so they are stack.

## Structs

structs allow multiple pieces of data to be stored together (kinda like tuples)

to define a struct like this

```rs
struct Student {
    name: String,
    age: u32,
    gpa: f64
}
```

you can initialize a struct like this

```rs
let nate = Student {
    name: String::from("Nate"),
    age: 20,
    gpa: 4.0
};
```

and use them like this

```rs
println!("Name: {} Age: {} GPA: {}", nate.name, nate.age, nate.gpa );
```

if you make it mutable you can set it like this

```rs
fn birthday(user: &mut Student) {
    user.age += 1;
}
```

structs muse be entirely mutable or entirely immutable there is no way to make only some fields mutable

## Methods

we can create a method my placing a function inside a `impl` block and s=making it's firs parameter `self`

```rs
impl Student {
    fn birthday(&mut self) {
        self.age += 1;
    }
}
```

the `impl` block is for implementation which includes methods

note: unlike C/C++ ypu can just do `.` and don't need to do `->` or `(*).` because rust automatically references and dereference things in some cases

## Associated Functions

you can also place functions inside `impl` that do not have a `self` they will be associated with the struct but not a method
(like static methods in Java)

```rs
impl Student {
    fn build_student(name: String, age: u32, gpa: f64) -> Student{
        Student {
            name: name,
            age: age,
            gpa: gpa
        }
    }
}
```

call an associated function using `::`

```rs
let mut nate = Student::build_student(String::from("Nate"), 20, 4.0);
```

## Error Handling

Rust does not have try, catch, or throw instead it has `Result`, `match`, and `panic!`

### Panic

when a rust program encounters a runtime error it is called a panic

you can manually trigger a panic like this

```rs
fn divide(a: f64, b: f64) -> f64 {
    if b == 0 {
        panic("divide by zero");
    } else {
        return a / b;
    }
}
```

if divide is called with b = 0 then the program will display "divide by zero" as an error and immediately exit with a non-zero exit code.

### Result

although panic is good for exiting the program there is no way to catch a panic

to solve this problem Rust has a built in enum called `Result`

`Result` is generic and takes the type of success and also the type of failure

in this example the function returns a 64-bit float if successful or a string if an error occurs

```rs
fn divide(a: f64, b: f64) -> Result<f64, &str> {
    if b == 0 {
        return Err("divide by zero");
    } else {
        return Ok(a / b);
    }
}
```

we can use a match statement to switch between failure and success

```rs
match divide(a, b) {
    Ok(value) => {
        println!("a/b: {}", value);
    },
    Err(message) => {
        println!("error: {}", message);
    }
}
```

you can also use a method called `unwrap` to automatically panic if the result is a failure

```rs
let c: f64 = divide(a, b).unwrap();
```
