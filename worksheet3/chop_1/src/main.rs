fn chop_1 (s : String) -> Vec<char> {
    let mut out: Vec<char> = Vec::new();
    
    for c in s.chars() {
        out.push(c);
    }

    return out;
}

fn main () {
    let s : String = String::from ("Hello, world!");
    println! ("{:?}", chop_1 (s.clone ()));
}