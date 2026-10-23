use std::io;

fn main() {
    let choice = choose_unit();
    let value;

    if choice == 1 {
	value = get_value("Celsius".to_string());
	let res = (value * 9.0 / 5.0) + 32.0;
	println!("{value} Celsius is {res} Fahrenheit");
    } else {
	value = get_value("Fahrenheit".to_string());
	let res = (value - 32.0) * 5.0 / 9.0;
	println!("{value} Fahrenheits is {res} Celsius");
    }
}

// 0 - to Fahrenheit
// 1 - to Celsius
fn choose_unit() -> i32 {
    println!("What conversion:\n\
	      [1] - Celsius to Fahrenheit\n\
	      [2] - Fahrenheit to Celsius\n\
	      [anything else] - quit.");

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Cannot read stdin");

    let choice = match line.trim().parse() {
	Err(_) => std::process::exit(0),
	Ok(v) => v
    };

    if choice == 1 || choice == 2 {
	return choice;
    }

    std::process::exit(0);
}
    
fn get_value(deg_name: String) -> f64 {
    loop {
	println!("plese enter {deg_name} value.");
	let mut line = String::new();
	io::stdin().read_line(&mut line).expect("cannot read stdin");
	match line.trim().parse() {
	    Ok(v) => { return v; }
	    Err(_) => {	continue; }
	};
    }
}
