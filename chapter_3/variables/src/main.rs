// Rust is  staticaly typed language, which means that it must know the type of all variables at
// compile time, even if it is inferred.

const MAX_POINTS: u32 = 100_000;

fn main() {
    // variables();
    compound_types();
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let diff = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // diviison
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}

fn variables() { 
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

    // Since here more than one type is possible, the declaration of type is mandatory
    let guess: u32 = "42".parse()
        .expect("Not a number");
    // // Statement below throws a compile error
    // let guess = "42".parse()
    //     .expect("Not a number");

    // In Rust there ar 4 scalar types that are: integer, floating-point, boolean, character
    // integer can be signed and unsigned, which basically means if a number can be negative or
    // positive, there the name "signed", because we would need a "-" to indicate a negative
    // number.
   
    // floating-point
    let w = 2.0; // f64 (default)
    let y: f32 = 3.0; // f32

    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // char
    let c = 'z';
    let z = 'Ω';
}

fn compound_types() {
    // Rust has 2 primitive compound types: tuples and arrays
    
    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    print!("The value of tup is {:?}\n", tup);
    
    let (x, y, z) = tup;
    print!("The value of z is {}\n", z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // arrays
    // arrays in Rust are a bit different then other languages, because in Rust an array has a
    // fixed length that cannot be changed once declared
    
    // A vector on the other hand can grow in size
    
    let a = [1,2,3,4,5];
    let first = a[0];
    let second = a[1];

    let index = 10;
    let element = a[index]; // index out of bounds
    print!("The value of element is {}\n", element);
}
