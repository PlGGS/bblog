fn find_num_occurrence_array_ref (n : u8, arr_ref : &[u8; 10]) -> usize {
    arr_ref.iter().filter(|&&x| x == n).count()
  }
  
  fn main () {
    let array = [4,5,6,7,8,9,5,5,6,10];
    for n in array {
        println!("There are {} occurance(s) of {} in the array", find_num_occurrence_array_ref(n, &array), n);
    }
  }