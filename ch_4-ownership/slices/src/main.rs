fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[2..4];

    println!("{:?}", a);
    println!("{:?}", slice);
}
