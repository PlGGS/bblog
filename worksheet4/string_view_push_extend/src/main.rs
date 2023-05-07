fn main() {
    let mut string: String = String::from ("hello");
    let mut vec: Vec<i32> = Vec::new();

    let char: char = ',';
    let int: i32 = 1000;

    string.push(char);
    println!("{}", string);
    
    string.push_str(" world!");
    println!("{}", string);
    
    vec.push(int);
    println!("{:?}", vec);
    
    vec.extend([1, 2, 3].iter());
    println!("{:?}", vec);
}
