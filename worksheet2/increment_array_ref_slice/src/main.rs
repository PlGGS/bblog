fn increment_array (mut arr : [u8; 10]) -> [u8; 10] {
    for i in 0..(arr.len()) {
        arr[i] += 1;
    }

    return arr;
}

fn increment_array_ref (arr_ref : &mut [u8; 10]) {
    for i in 0..(arr_ref.len()) {
        arr_ref[i] += 1;
    }
}

fn increment_slice (slice : &mut [u8]) {
    for i in 0..(slice.len()) {
        slice[i] += 1;
    }
}

fn main () {
    let array : [u8; 10] = [4,5,6,7,8,9,5,5,6,10];
    dbg! (array);
    dbg! (increment_array (array));

    let mut array1 = [0; 10];
    array1.copy_from_slice(&array[..]);
    dbg! (array1);
    dbg! (increment_array_ref (&mut array1));
    dbg! (array1);

    let mut array2 = [0; 10];
    array2.copy_from_slice(&array[..]);
    dbg! (array2);
    dbg! (increment_slice (&mut array2));
    dbg! (array2);
}
  