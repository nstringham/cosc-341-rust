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
