use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main () -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = env::args ().skip (1).collect ();
    let input_filename : &str = &args[0];
    let input : File = File::open (input_filename)?;
    
    let metadata = input.metadata()?;
    let input_length: usize = metadata.len() as usize;
    
    let offset: usize = 16;

    let data : String = std::fs::read_to_string (input_filename)?;
    let d_vec: Vec<char> = data.replace("\n", ".").chars().collect();
    for (i, b) in input.bytes().enumerate() {
        let b : u8 = b?;

        if i % offset == 0 {
            print!("{}  ", zero_padded(format!("{:x}", i)));
        }

        print!("{}", format!("{:02x}", b));

        if (i % offset + 1) % (offset / 8) == 0 {
            print!(" ");
        }

        if i % 16 == 15 || i == input_length - 1 {
            print!(" |");

            for o in i + 1 - offset..i + 1 {
                print!("{}", d_vec[o]);
            }

            println!("|");
        }
    }
    Ok (())
}

fn zero_padded(s: String) -> String {
    let mut out: String = String::new();

    for _ in 0..8 - s.len() {
        out.push('0');
    }

    out = out + &s;
    return out;
}
