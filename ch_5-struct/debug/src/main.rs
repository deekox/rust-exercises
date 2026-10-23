#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    male: bool
}



fn main() {
    let p1 = Person {
	name: String::from("Marek"),
	age: 46,
	male: true
    };

    
    println!("{:?}", p1);
    println!("{:#?}", p1);
}
