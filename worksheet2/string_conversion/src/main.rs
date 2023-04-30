#![allow(dead_code)]

fn string_to_vector (s : &str) -> Vec<u8> {
    return s.bytes ().collect::<Vec<_>> ();
}

fn vector_to_string (v : &[u8]) -> String {
    return String::from_utf8_lossy (v).into_owned ();
}

fn main () {
    let string = "Hello, World!";
    dbg!(string_to_vector(&string));
    dbg!(vector_to_string(string.as_bytes()));
}