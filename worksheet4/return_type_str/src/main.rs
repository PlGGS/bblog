//we can't write f1 with the return type &str, because every reference must have a specified lifetime in Rust
//with no input params to borrow and return back to their owners, we can't return a reference to a string slice (&str) back to main
// fn f1 () -> &str {
fn f1 () -> &'static str {
    return "Hello, world!";
}

//Similarly, we also can't write f2 with the return type &str, because every reference must have a specified lifetime in Rust
// fn f2 (s : String) -> &str {
fn f2 (s : String) -> &'static str {
    let boxed_str = Box::new(s);
    let static_str = Box::leak(boxed_str);
    return &*static_str;
}

//it's totally fine in this case, because r is borrowed from main
fn f3 (r : &String) -> &str {
    return r;
}

//same thing as f3 here, because r is borrowed from main
fn f4 (r : &str) -> &str {
    return r;
}

fn main() {
    println!("{}", f1());
    
    let s: String = String::from("Hello, world!");
    println!("{}", f2(s));

    let r: String = String::from("Hello, world!");
    println!("{}", f3(&r));

    println!("{}", f4("Hello, world!"));
}
