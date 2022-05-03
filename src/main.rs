extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main(){
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is : {} ", secret_number);

    println!("Guess the number!");
    let mut guess = String::new();
    let scanner = io::stdin();

    println!("Enter your guess...");

    let num_bytes = scanner.read_line(&mut guess)
           .expect("Failed to read line");
    let guess: u32 = guess
                          .trim()
                          .parse()
                          .expect("failed to parse guess into a string");
                          
    println!("The number you entered : {}", guess);
    println!("The number of bytes is : {}", num_bytes);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Your guess is less than the number"),
        Ordering::Equal => println!("Your guess is correct and equal to the number"),
        Ordering::Greater => println!("Your guess is greater than the number"),

    }
}