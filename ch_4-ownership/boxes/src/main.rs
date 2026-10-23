fn main() {
    let a = Box::new(3);
    println!("{a}");

    let b =3;
    oj(b);
    println!("{b}");
}

fn oj(i: i32) -> () {
    println!("{i}");
}
