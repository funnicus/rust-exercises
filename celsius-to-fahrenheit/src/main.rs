use std::io;

fn main() {
    println!("Convert celsius to fahrenheit and vice versa!");
    println!("From which to which do you want to convert? Type c for from celsius and f for from fahrenheit...");

    let mut answer = String::new();

    loop {
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line!");
        if answer.trim() == "c" || answer.trim() == "f" { 
            break;
        }
    }

    if answer.trim() == "c" {
        println!("Converting from celsius to fahrenheit...");
        loop {
            println!("Type your temperature:");
            let mut number = String::new();
            io::stdin()
                .read_line(&mut number)
                .expect("Failed to read line!");
            let mut number: i32 = match number.trim().parse() {
                Ok(number) => number,
                Err(_) => continue,
            };

            number = number * 2 + 30;
            println!("Your temperature in fahrenheit is: {}", number);
            break;
        }
    }
    else {
        println!("Converting from fahrenheit to celsius...");
        loop {
            println!("Type your temperature:");
            let mut number = String::new();
            io::stdin()
                .read_line(&mut number)
                .expect("Failed to read line!");
            let mut number: i32 = match number.trim().parse() {
                Ok(number) => number,
                Err(_) => continue,
            };

            number = (number - 30)/2;
            println!("Your temperature in celsius is: {}", number);
            break;
        }
    }
}
