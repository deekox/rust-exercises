use rand::Rng;

enum Thing {
    Nothing,
    Struct { name: String, age: i32, },
}

impl Thing {
    fn describe(&self) {
	match self {
	    Self::Struct {name, age} => println!("name: {name}, age: {age}"),
	    Self::Nothing => ()
	}
    }
}

fn main() {
    let x = Thing::Nothing;
    let y = Thing::Struct {name: String::from("Dupa"), age: 32};

    x.describe();
    y.describe();

    let mut rng = rand::rng();

    for _ in 1..=5 {
	let w = rng.random_range(0..=3);
	match w {
	    2 => println!("U2"),
	    1 => println!("w is top one"),
	    other => println!("other: {other}"),
	}
    }

    let zx = 81;
    match zx {
	81 => println!("one and only"),
	other => println!("other value (not 81)"),
    }    
}

struct TupleStruct(u8, u8);
