fn chop_3 (s : String) -> Vec<[char; 3]> {
    let mut out: Vec<[char; 3]> = Vec::new();

    let s_vec: Vec<char> = s.chars().collect();
    let mut chunk: [char; 3] = [' '; 3];
    for i in 0..s.len() {
        match i % 3 {
            0 => {
                chunk[0] = s_vec[i];
            },
            1 => {
                chunk[1] = s_vec[i]
            },
            2 => {
                chunk[2] = s_vec[i];
                out.push(chunk);
                chunk = [' '; 3];
                continue;
            },
            _ => println!("Huh?")
        }
        
        if i == s_vec.len() - 1 {
            out.push(chunk);
        }
    }

    return out;
}

fn main () {
    let s : String = String::from ("Hello, world!");
    println! ("{:?}", chop_3 (s.clone ()));
}
