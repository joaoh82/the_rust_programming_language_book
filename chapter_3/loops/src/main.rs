// Rust has three kinds of loops: loop, while, for.

fn main() {
    // loop {
    //     print!("infinite loop\n");
    // }
    
    let mut number = 0; 
    loop {
        print!("The value of number is {}\n", number);
        number = number + 1;
        if number == 11 {
            break; // break keyworkd stops the loop
        }
    }

    // while

    let mut new_number = 3;
    while new_number != 0 {
        print!("{}!\n", new_number);
        new_number = new_number - 1;
    }
    print!("LIFTOFF!!!\n");

    // same solution, different approach
    for number in (1..4).rev() {
        print!("rev: {}!\n", number);
    }
    print!("rev: LIFTOFF!!!\n");

    // looping through an array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        print!("while: the value is {}\n", a[index]);
        index = index + 1;
    }
    
    // for

    // same solution with a for loop
    for element in a.iter() {
        print!("for: the value is {}\n", element);
    }
}
