use std::env;
use std::error::Error;

//This fails on PDFs and executables, because those files contain characters outside UTF-8
fn main () -> Result<(), Box<dyn Error>> {
  let args : Vec<String> = env::args ().skip (1).collect ();
  let input_filename : &str = &args[0];
  let data : String = std::fs::read_to_string (input_filename)?;
  println! ("{}", reverse_string(data));
  Ok (())
}

fn reverse_string (s : String) -> String {
    let mut out: Vec<char> = Vec::with_capacity(s.len());
    
    let s_vec: Vec<char> = s.chars().collect();
    for i in 0..s.len() {
        out.push(s_vec[s.len() - 1 - i]);
    }

    return out.iter().collect();
}
