fn main() {

    let mut x = 3;
    let a = loop {
	if x == 0 { break 123 }
	println!("x: {x}");

	x -= 1;
    };

    println!("a: {a}");
}
