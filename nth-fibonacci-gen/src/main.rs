use std::io;

fn main() {
    let mut f1: i64 = 0;
    let mut f2: i64 = 1;
    let mut result: i64 = 1;

    println!("Welcome to nth fibonacci gen!");
    println!("Write a positive number:");
    loop {
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line!");
        let number: i32 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => panic!("Something went wrong with read line!"),
        };
        if number < 0 {
            continue;
        }
        println!("Generating {}:th fibonacci...", number);
        if number == 0 { 
            println!("0th fibonacci is 0");
            break;
        }
        if number == 1 { 
            println!("1st fibonacci is 1");
            break;
        }
        for i in 0..number{
            if i == 0 || i == 1 { continue; }
            result = f1 + f2;
            f1 = f2;
            f2 = result;
            println!("{}", result);
        }
        println!("Your {}th fibonacci is: {}", number, result);
        break;
    }
}
