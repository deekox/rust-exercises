enum Color {
    None,		
    RGB(i32, i32, i32),
    Text{ name: String, value: i32}
}

impl Color {
    fn describe(&self) {
 	match self {
	    Color::None => {
		println!{"None"};
	    }
	    Self::RGB(a, b, c) => {
		println!{"RGB({a}, {b}, {c})"};
	    }
	    Self::Text{name, value} => {
		println!{"Text.name: {name}, .value: {value}"};
	    }
	}
    }

    fn change(&mut self) {
	match self {
	    Self::Text{name, value: _} => {
		name.push_str(" <<mutated>>");
	    }
	    _ => ()
	}
    }
}

fn get(s: &mut String) {
    let r = &mut *s;

    println!("{s}");

    r.push_str("ojej");
}

fn main() {
    let c1 = Color::None;
    let c2 = Color::RGB(255, 0, 0);
    let mut c3 = Color::Text{ name: String::from("zolty"), value: 200 };


//    let x: Option<String> = Some(String::from("dupa"));

    let x = None;
    
    let y = & match &x {
	Some(a) => { println!("String {a}"); a }
	None => { println!("None"); &String::from("match arm") }
    };

    println!("y is {y}");
    
    
    c3.describe();
    c3.change();
    c3.describe();

}
