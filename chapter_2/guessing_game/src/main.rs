extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the numeber!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is {}", secret_number);
   
    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line:");

        // Rust allows shadowing of already created variables. This situation is often used in
        // situations in which you want to convert a value from on type to another. 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);

        // A match expression is made up of arms. An arm consists of a pattern and the code that
        // should be run if the value given to the beginning of the match expression fits that
        // arm's pattern
        match guess.cmp(&secret_number) {
             Ordering::Less => println!("Too small!"),
             Ordering::Greater => println!("Too big!"),
             Ordering::Equal => {
                 println!("You win!");
                 break;
             }

        }        
    }
}
