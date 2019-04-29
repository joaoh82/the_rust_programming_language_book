fn main() {
    print!("Result - recursive: {}\n", nth_fibonacci_recursive(50)); 
    // time - 104.31 real       103.63 user         0.30 sys

    print!("Result - dynamic: {}\n", nth_fibonacci_dynamic(50));
    // time - 0.82 real         0.46 user         0.22 sys
}

fn nth_fibonacci_recursive(n: i32) -> u64 {
    if n <= 2 {
        1 
    } else {
        nth_fibonacci_recursive(n-1) + nth_fibonacci_recursive(n-2)
    }
}

fn nth_fibonacci_dynamic(n: usize) -> u64 {
    let mut f: Vec<u64> = Vec::with_capacity(n+1);
    f.push(1);
    f.push(1);
    let mut index = 2;
    while index < n {
        let push = f[index-1].wrapping_add(f[index-2]);
        f.push(push);
        index = index + 1;
    }
    f[n-1]
}
