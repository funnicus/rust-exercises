use std::io;

/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
Keep in mind the details about UTF-8 encoding!
*/

fn main() {
    println!("The pig-latin converter!");
    println!("Enter a word:");

    let mut word = String::new(); //create a new mutable empty String. Variables are inmutable by default in Rust.
    
    io::stdin()
        .read_line(&mut word) //mutable refrence to guess, we don't want to transfer ownership!
        .expect("Failed to read line"); //expect causes the program to crash if io::Result returned from .read_line is an Err value
    println!("Your word in pig-latin is:");
    match word.chars().nth(0).unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'ä' | 'ö' | 'A' | 'E' | 'I' | 'O' | 'U' | 'Y' | 'Ä' | 'Ö' => println!("{}-hay", word.trim()), //if match print word-hay
        c => println!("{}-{}ay ", &word[c.len_utf8()..].trim(), c) //else print everycharfromc-cay
    }
}