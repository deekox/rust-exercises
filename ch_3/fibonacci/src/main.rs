use std::io;

fn fibo(nth : u64) -> u64 {
    if nth == 0 {
	return nth
    }
    if nth == 1 {
	return nth
    }

    fibo(nth - 2) + fibo(nth - 1)
}
    
fn main() {

    loop {
	println!("What Finbonacci's number do you want to see?");
	
	let mut line = String::new();
	io::stdin().read_line(&mut line).expect("cannot read stdin");

	let nth: u64 = match line.trim().parse() {
	    Err(_) => {
		println!("Please provide non-negative integer");
		continue;
	    }
	    Ok(v) => v
	};

	println!("fibo({nth}) = {}", fibo(nth));
	break;
    }
}
