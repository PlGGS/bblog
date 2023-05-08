use std::str::FromStr;
use std::error::Error;
use std::io::Write;

fn main () {
  let name: String = get_name("What is your name?\n> ").unwrap();
  println! ("Hello {name}!");
  let age: u32 = get_age("What is your age?\n> ").unwrap();
  println!("You are {age} years old.");
  println! ("Goodbye {name}!");
}

fn get_name(prompt: &str) -> Result<String, Box<dyn Error>> {
    print! ("{}", prompt);
    // print! ("> ");
    std::io::stdout ().flush ()?;
    let mut name : String = String::with_capacity (64);
    let _num_bytes_read = std::io::stdin ().read_line (&mut name)?;
    return Ok(name.trim().to_string());
}

fn get_age(prompt: &str) -> Result<u32, Box<dyn Error>> {
    print! ("{}", prompt);
    // print! ("> ");
    std::io::stdout ().flush ()?;
    let mut age : String = String::with_capacity (64);
    let _num_bytes_read = std::io::stdin ().read_line (&mut age)?;
    let parsed_age: u32 = u32::from_str(age.trim())?;
    return Ok(parsed_age);
}
