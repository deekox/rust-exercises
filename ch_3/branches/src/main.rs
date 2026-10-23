use std::io;

fn main() {

    let number = loop {
	println!("Please provide a number");

	let mut line = String::new();
	io::stdin()
	    .read_line(&mut line)
	    .expect("cannot read stdio");
	
	number: i32 = match line.trim().parse() {
	    Err(_) => continue,
	    Ok(v) => v
	};
	break number;
    };

    let was_it_tree = if number == 3 {
	println!("it's a tree");
	true
    } else {
	println!("sad face :(");
	false
    };

    println!("was_it_tree: {was_it_tree}");
}

