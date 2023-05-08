fn f (r : &str) -> &str {
    &r[2..]
}

fn main () {
    dbg! (f ("hello")); //"hello" is a reference to a string slice (&str) 
    dbg! (f (&String::from ("hello"))); //&String::from ("hello") is a reference to full fat String object, which is being interpreted as a string slice (&str) by f
}