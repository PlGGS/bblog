fn main() {
    let mut vec = Vec::<u8>::with_capacity(3);

    vec.push(1);
    vec.push(2);
    vec.push(3);

    dbg!(vec.len());
    dbg!(vec.capacity());
    dbg!(vec);
}
