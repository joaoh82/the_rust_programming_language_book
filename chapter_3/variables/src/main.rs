// Rust is  staticaly typed language, which means that it must know the type of all variables at
// compile time, even if it is inferred.

const MAX_POINTS: u32 = 100_000;

fn main() {
    // Variable a immutable by default in Rust, so to make a variable mutable we need to use the
    // keyword "mut"
    let mut x = 5;
    print!("The value of x is: {} \n", x);
    x = 6;
    print!("The value of x is: {}\n", x);

    // Shadowing
    let y = 6;
    let y = y + 1;
    let y = y * 2;
    print!("The value of y = {}\n", y);

    // Sicne here more than one type is possible, the declaration of type is mandatory
    let guess: u32 = "42".parse()
        .expect("Not a number");
    // // Statement below throws a compile error
    // let guess = "42".parse()
    //     .expect("Not a number");


}
