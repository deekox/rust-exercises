use std::io;

fn main() {
    let ar = [1, 2, 3, 4, 5];
    let mut index = String::new();

    loop {
	println!("Please provide the index");

	io::stdin().read_line(&mut index).expect("błąd odczytu stdin");


	// HERE index is shadowed by index
	// but when the second loop iteration comes the
	// index should be of usize type? or what
	// 
	
	let index: usize = match index.trim().parse() {
	    Err(_) => {
		println!("To ma być liczba!");
		println!("Podałeś \"{index}\"");
		continue;
	    }
	    Ok(v) => v
	};
	println!("Pod indeksem {index} kryje się:");
	println!("{}", ar[index]);
	break;
    }
}
