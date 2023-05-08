//&[&str]
fn join_string_slice_array_str () {
    let array : [&str; 4] = ["the", "rain", "in", "Spain"];
    let slice : &[&str] = &array;
    let s : String = slice.join (" ");
    println! ("{s}");
}

//&[i32]
fn join_vector_slice_array_i32 () {
    let array : [&[i32]; 3] = [ &[1,2], &[3,4,5], &[6,7,8,9] ];
    let slice : &[&[i32]] = &array;
    let v : Vec<i32> = slice.join::<&[i32]> (&[-2, -1]);
    println! ("{v:?}");
}

//&[String]
fn join_string_slice_array_string () {
    let array: [String; 4] = [String::from("the"), String::from("rain"), String::from("in"), String::from("Spain")];
    let slice: &[String] = &array;
    let s: String = slice.join(" ");
    println!("{s}");
}

//Vec<String>
fn join_string_slice_vec_string () {
    let mut vec: Vec<String> = Vec::new();

    vec.push(String::from("the"));
    vec.push(String::from("rain"));
    vec.push(String::from("in"));
    vec.push(String::from("Spain"));

    let slice: &[String] = &vec;
    let s: String = slice.join(" ");
    println! ("{s}");
}

//Vec<&str>
fn join_string_slice_vec_str () {
    let mut vec: Vec<&str> = Vec::new();

    vec.push("the");
    vec.push("rain");
    vec.push("in");
    vec.push("Spain");
    
    let slice: &Vec<&str> = &vec;
    let s: String = slice.join(" ");
    println! ("{s}");
}

fn main () {
    join_string_slice_array_str();
    join_vector_slice_array_i32();
    
    join_string_slice_array_string();
    join_string_slice_vec_string();
    join_string_slice_vec_str();
}