fn f (s : String) -> String {
    let mut x: String = s.clone();
    x.push_str("world");
    return x;
}

fn g (s : String) -> &'static str {
    let mut boxed_str = Box::new(s);
    boxed_str.push_str("world");
    let static_str = Box::leak(boxed_str);
    return &*static_str;
}

fn main () {
    let orig = String::from ("hello");
    dbg! (f (orig.clone ()));
    dbg! (g (orig.clone ()));
}
