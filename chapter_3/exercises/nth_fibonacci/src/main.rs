fn main() {
    print!("Result: {}\n", nth_fibonacci(10)); 
    // 1, 1, 2, 3, 5, 8, 13, 21, 34, 55
}

fn nth_fibonacci(n: i32) -> i32 {
    if n <= 2 {
        1 
    } else {
        nth_fibonacci(n-1) + nth_fibonacci(n-2)
    }
}
