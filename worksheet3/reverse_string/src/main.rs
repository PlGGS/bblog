fn reverse_string (s : String) -> String {
        let mut out: Vec<char> = Vec::with_capacity(s.len());
        
        let s_vec: Vec<char> = s.chars().collect();
        for i in 0..s.len() {
            out.push(s_vec[s.len() - 1 - i]);
        }

        return out.iter().collect();
}

fn main () {
    let s : String = String::from ("Hello, world!");
    println! ("{}", reverse_string (s));
}