use std::mem;

struct Point (i32, i32, i32);
struct Trait;

struct User {
    name: String,
    age: u64,
    city: String,
}


fn main() {
    let age = 32;
    let name = String::from("Marek");    

    let u1 = User {
	name,
	age,
	city: String::from("New York"),
    };

    let u2 = User {
	age: 33,
	..u1
    };
        
    println!("{} {} {}", u2.name, u2.age, u2.city);


    let z = Point(11, 22, 33);
    println!("{} {} {}", z.0, z.1, z.2);
    println!("size_of::<User>(): {}", size_of::<User>());

    let y = (22, 33, 44);
    println!("{} {} {}", y.0, y.1, y.2);

    let (a, b, c) = y;
    println!("{} {} {}", a, b, c);
    println!("size_of_val(&a){}", mem::size_of_val(&a));
        

    let x = Trait;
    show_size(x);
}

fn show_size(x: Trait) {
    
    println!("{}", mem::size_of::<Trait>());
    println!("{}", mem::size_of_val(&x));
}
