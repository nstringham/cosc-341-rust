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

fn compute_sqrt(x: f64) -> f64 {
    let mut guess: f64 = 1.0;

    for _ in 0..10 {
        guess = 0.5 * (guess + x / guess);
    }

    guess
}

// true at index 2, 3, 5, 7 (the prime indexes)
const FIRST_8_IS_PRIME: [bool; 8] = [false, false, true, true, false, true, false, true];

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
