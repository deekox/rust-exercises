struct Point {
    a: i32,
    b: i32,
}

fn main() {
    let mut p = Point {
	a: 12,
	b: 66,
    };

    let mr = &mut p.a;
    let x = &mut p.b;
    
    println!("{}", mr);
}
