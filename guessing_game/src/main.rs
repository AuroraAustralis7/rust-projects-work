use std::io;
use std::cmp::Ordering;
use rand::RngExt;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(0..=100);
    /*
    thread_rng() has been renamed to rng(), 
    gen_range() has been renamed to random_range(),
    use rand::RngExt for the convenient helpers.
     */

    // println!("The secret number is {secret_number}");

    loop {
    println!("Please input your numerical guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) // adding whitespace makes it more readable, but removing it would still work.
        .expect("Failed to read line");

    // io::stdin().readline(&mut guess).expect("Failed to read line") this line still works

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => { // _ is placeholder that accepts anything
            println!("Input a NUMBER, please.");
            continue;
        },
    };
    
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too low"),
        Ordering::Equal => {
            println!("correct!");
            break;
        },
        Ordering::Greater => println!("too high"),
    }
}
}
