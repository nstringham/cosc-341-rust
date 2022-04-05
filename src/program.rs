use std::io;
use std::io::Write;
use std::fs;
/// computes the value of pi using n terms of an infinite sequence
/// 
/// this function uses the [Leibniz formula for π]
/// 
/// # Arguments
/// * `n` - number of terms to compute
/// 
/// [Leibniz formula for π]: https://en.wikipedia.org/wiki/Leibniz_formula_for_π
fn compute_pi(n: i32) -> f64 {
    // this sum approaches pi/4
    let mut sum: f64 = 0.0;

    let max_denominator = n * 2;
    

    let mut denominator = 1;
    let mut numerator = 1.0;
    while denominator < max_denominator {
        sum += numerator / denominator as f64;

        numerator *= -1.0;
        denominator += 2;
    }

    // 4 * pi/4 = pi
    sum * 4.0
}

/// computes the square root of a number using 10 iterations of [Newton's method]
/// 
/// # Arguments
/// * `x` - the number to find the root of
/// 
/// [Newton's method]: https://en.wikipedia.org/wiki/Newton%27s_method#Square_root
fn compute_sqrt(x: f64) -> f64 {
    let mut guess: f64 = 1.0;

    for _ in 0..10 {
        guess = 0.5 * (guess + x / guess);
    }

    guess
}

/// true at index 2, 3, 5, 7 (the prime indexes)
const FIRST_8_IS_PRIME: [bool; 8] = [false, false, true, true, false, true, false, true];

/// checks wether a given number is prime
/// 
/// # Arguments
/// * `n` - the number to check
fn is_prime(n: i32) -> bool {
    
    // we only need to check if a number is divisible by a prime numbers
    //
    // all prime numbers greater than 3 are either 6x+1 or 6x-1 where x is a whole number
    //
    // if n is not divisible by any number less than sqrt(n) then it is prime
    //
    // so if n is 101 we can just check if n is divisible by 2, 3, 5, 7, 11, and 14
    

    // negative numbers are not prime
    if n < 0 {
        return false;
    }

    // use lookup table for 0-7 because it's fast and the main algorithm doesn't work for 7 and below
    if n < 8 {
        return FIRST_8_IS_PRIME[n as usize];
    }

    // check if it's divisible by 2 or 3
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    // k is a multiple of 6
    let mut k = 6;

    loop {
        // if it is divisible by 6x+1 or 6x-1
        if n % (k - 1) == 0 || n % (k + 1) == 0 {
            return true;
        }

        // if we have checked all the factors less than sqrt(n)
        if k * k > n {
            return true;
        }

        k += 6;
    }
}

/// prints all the primes less than or equal to a given number
/// 
/// # Arguments
/// * `n` - the max prime to print
fn display_primes(n: i32) {
    for i in 0..n {
        if is_prime(i) {
            if i > 2 {
                print!(", ");
            }

            print!("{}", i);
        }
    }

    println!();
}

/// processes score info from stdin
/// 
/// reads students and grades from stdin until a q is reached then prints
/// average score, minimum score, maximum score, and name of minimum and maximum scores
fn process_scores() {
    print!("how many grades: ");
    let n: u32 = read().unwrap();

    let mut count = 0;
    let mut total_score = 0;

    
    let mut min_score = 0;
    let mut max_score = u32::MAX;
    let mut min_name = String::new();
    let mut max_name = String::new();

    for i in 0..n {
        print!("student {} name: ", i + 1);
        std::io::stdout().flush().unwrap();
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();

        print!("student {} grade: ", i + 1);
        let score: u32 = read().unwrap();

        total_score += score;
        count += 1;

        // if score is new minimum
        if score < min_score
        {
            min_score = score;
            min_name = name;
        }

        // if score in new maximum
        else if score > max_score
        {
            max_score = score;
            max_name = name;
        }
    }

    println!();

    print!("average score: {}", total_score as f64 / count as f64);
    print!("minimum score: {} {}", min_score, min_name);
    print!("maximum score: {} {}", max_score, max_name);
}
/// solves the quadratic equation ax^2 + bx + c = 0
/// 
/// this function uses mutable references to "return" multiple values
///
/// # Arguments
/// * `a` - the a coefficient
/// * `b` - the b coefficient
/// * `c` - the c coefficient
/// * `solution1` - the address to store the first solution in
/// * `solution2` - the address to store the second solution in
///
/// # Returns
/// true if there is a solution, false if there is no solution
fn quadratic_mut(a: f64, b: f64, c: f64, x1: &mut f64, x2: &mut f64) -> bool {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        false
    } else {
        let root = compute_sqrt(discriminant);
        *x1 = (-b + root) / (2.0 * a);
        *x2 = (-b - root) / (2.0 * a);
        true
    }
}

/// solves the quadratic equation ax^2 + bx + c = 0
/// 
/// this function uses a tuple to return multiple values
/// 
/// this function returns `None` if there is not solution
///
/// # Arguments
/// * `a` - the a coefficient
/// * `b` - the b coefficient
/// * `c` - the c coefficient
fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    } else {
        let root = compute_sqrt(discriminant);
        Some(((-b + root) / (2.0 * a), (-b - root) / (2.0 * a)))
    }
}

/// finds the sum of the squares of the numbers from 1 to n
/// 
/// this function is implemented using recursion
/// 
/// # Arguments
/// * `n` - a positive integer
/// # Returns
/// 1 + 4 + 9 + 16 + ... + n<sup>2</sup>
fn sum_squares(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        n * n + sum_squares(n - 1)
    }
}

/// counts the number of characters blanks and lines in a file
/// 
/// # Arguments
/// * `file_name` -  the name of the file to count blanks and lines in
/// 
/// # Returns
/// * the number of characters in the file
/// * the number of blanks in the file
/// * the number of lines in the file
fn file_count(file_name: &str) -> (u32, u32, u32) {
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut characters = 0;
    let mut blanks = 0;
    let mut lines = 1;

    for c in contents.chars() {
        if c == '\n' {
            lines += 1;
        } else if c == ' ' {
            blanks += 1;
        }

        characters += 1;
    }

    (characters, blanks, lines)
}

/// reads a value from stdin
/// 
/// this requires that the user only inputs one item per line
fn read<T: std::str::FromStr>() -> Result<T, T::Err> {
    // flush in case we use `print!()` before `read()`
    io::stdout().flush().unwrap();

    // read a line of text
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // remove whitespace and convert to the requested return type
    input.trim().parse()
}
