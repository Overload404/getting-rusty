fn main() {
    let n = 30;
    let outcome = fibonacci_recursive(n);
    println!("Then {}-th number in fibonacci sequense is: {}", n, outcome);
}

fn fibonacci_recursive(num: u64) -> u64 {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    } else {
        return fibonacci_recursive(num - 1) + fibonacci_recursive(num - 2);
    }
}