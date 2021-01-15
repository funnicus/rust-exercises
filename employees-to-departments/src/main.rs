/*
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a 
department or all people in the company by department, sorted alphabetically.
*/

use std::io;
use std::collections::HashMap;

fn main() {
    println!("Add employees to departments!");
    println!("Enter a word:");

    let mut data_base: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut employee = String::new(); //create a new mutable empty String. Variables are inmutable by default in Rust.
        let mut department = String::new();

        println!("Type 'q' to quit");
        println!("Type employee name:");
        
        io::stdin()
            .read_line(&mut employee) //mutable refrence to employee, we don't want to transfer ownership!
            .expect("Failed to read line"); //expect causes the program to crash if io::Result returned from .read_line is an Err value

        println!("Type department name:");

        io::stdin()
            .read_line(&mut department)
            .expect("Failed to read line");

        if employee.trim() == "q" || department.trim() == "q" { break; }

        data_base.entry(String::from(department.trim())).or_insert(Vec::new()).push(String::from(employee.trim()));
    }

    println!("Next type department name to list all the people in that department or...");
    println!("type 'employees' to list all the people in our company...");

    let mut selection = String::new();

    io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

    if selection.trim() == "employees" {
        println!("Listing all employees and departments:");
        for (d, mut e) in data_base {
            e.sort();
            println!("There is {:?} from {}", e, d);
        }
    }
    else {
        println!("All employees on the department {}:", selection.trim());
        for (d, mut e) in data_base{
            if d != selection.trim() { continue; }
            e.sort();
            println!("There is {:?} from {}", e, d);
        }
    }

}
