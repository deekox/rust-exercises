fn main() {
    let s = String::from("Hello");
    consume(s);
    //    println!("{s}");

    let i = 23;
    consume_int(i);
    println!("{i}");
}

fn consume(_s: String) {
}
    
fn consume_int(_i: i32) {
}
