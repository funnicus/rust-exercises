use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); //generating a secret number with rand crate

    loop { //creating an infinite loop with loop
        println!("Please input your guess.");

        let mut guess = String::new(); //create a new mutable empty String. Variables are inmutable by default in Rust.
    
        io::stdin()
            .read_line(&mut guess) //mutable refrence to guess
            .expect("Failed to read line"); //expect causes the program to crash if io::Result returned from .read_line is an Err value
    
            //shadowing guess variable with a new value, beacause we are converting a type to another
            //let's make the guess an unsigned, 32 bit integer by trimming and parsing the input string
            let guess: u32 = match guess.trim().parse() { //errorhandling with match
                Ok(num) => num, //return the number if input is correct
                Err(_) => continue, //otherwise skip to next loop iteration. _ is an catchall value for all Error values
            };
    
        println!("You guessed: {}", guess); //{} hold the value of guess within them
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; //breaking the loop
            },
        }
    }
}