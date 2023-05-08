fn f (r : &String) -> &str {
    // println!("{}", &r[2..]);
    &r[2..]
}

fn main () {
    //the line below would fail to compile, because f expects a reference to a full fat String object, not a reference to a string slice (or an immutable string of utf-8 chars)
    // dbg! (f ("hello"));
    
    //instead, we pass a reference to a fully-fledged String object
    let string: String = String::from("hello");
    dbg! (f (&string));
}