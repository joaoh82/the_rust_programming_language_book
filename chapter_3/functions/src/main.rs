fn main() {
    another_function(5, 6);

    let x = 5;
    let y = {
        let x = 3;
        x + 1 // no semicolon here, returning y
    };
    print!("main: The value of y is {}\n", y);

    let five = five();
    print!("The value of five is {}\n", five);

    let plus_one = plus_one(5);
    print!("The value of plus_one is {}\n", plus_one);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: i32) {
    print!("The value of x is {}\n", x);
    print!("The value of y is {}\n", y);
}
