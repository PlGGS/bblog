fn f1 (r : &str) -> [&str; 3] {
    [&r[0..4], &r[4..8], &r[8..]]
}

//impossible
// fn f2 (r : &str) -> &[&str] {
// }

fn f3 (r : &str) -> Vec<&str> { 
    let mut out: Vec<&str> = Vec::new();
    
    out.push(&r[0..4]);
    out.push(&r[4..8]);
    out.push(&r[8..]);

    return out;
}

fn g1 (r : &str) -> [String; 3] {
    [String::from(&r[0..4]), String::from(&r[4..8]), String::from(&r[8..])]
}

//impossible
// fn g2 (r : &str) -> &[String] {
// }

fn g3 (r : &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    
    out.push(String::from(&r[0..4]));
    out.push(String::from(&r[4..8]));
    out.push(String::from(&r[8..]));

    return out;
}

fn main () {
    dbg! (f1 ("the rain in Spain"));
    // dbg! (f2 ("the rain in Spain"));
    dbg! (f3 ("the rain in Spain"));
    dbg! (g1 ("the rain in Spain"));
    // dbg! (g2 ("the rain in Spain"));
    dbg! (g3 ("the rain in Spain"));
}