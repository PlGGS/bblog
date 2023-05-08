use std::num::ParseIntError;
use std::str::FromStr;
use std::error::Error;
use std::io::Write;

fn main () {
    let name: String = get_from_stdin("name").unwrap();
    println! ("Hello {name}!");

    let age_string: String = get_from_stdin("age").unwrap();
    let parsed_age: Result<u32, ParseIntError> = u32::from_str(age_string.trim());
    if let Err(e) = parsed_age {
        println!("{:?}", e);
        return;
    }
    
    let age: u32 = parsed_age.unwrap();
    println!("You are {age} years old.");
    println! ("Goodbye {name}!");
}

fn get_from_stdin(val_to_get: &str) -> Result<String, Box<dyn Error>> {
    let mut value : String = String::with_capacity (64);
    
    while value.trim().is_empty() {
        print!("What is your {val_to_get}?\n> ");
        std::io::stdout ().flush ()?;
        std::io::stdin ().read_line (&mut value)?;
    }
    
    return Ok(value.trim().to_string());
}
