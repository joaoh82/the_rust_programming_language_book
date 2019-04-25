fn main() {
    let number = 8;
    
    if number < 5 {
        print!("condition was true\n");
    } else {
        print!("condition was false\n");
    }

    // if number {
    //     print!("This if statement throws an error");
    // }
   
    if number != 0 {
        print!("number was anything other then zero\n");
    }

    if number % 4 == 0 {
        print!("number is divisible by 4\n");
    } else if number % 3 == 0 {
        print!("number is divisible by 3\n");
    } else if number % 2 == 0 {
        print!("number is divisible by 2\n");
    } else {
        print!("number is not divisible by 4. 3 or 2\n");
    }

    let condition = true;
    let number = if condition {
        5
    } else { 
        6
    };

    print!("The value of number is {}\n", number);

    // This throws an incompatible types error
    // let number = if condition {
    //     5   
    // } else {
    //     "six"
    // };
}
