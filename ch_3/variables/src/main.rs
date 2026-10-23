const X_TO_X: u32 = 3;

fn main() {
    let pi = "3.14159265358979dupa";
    let pi: f64 = match pi.parse() {
	Err(_) => {
	    println!("nie udało się sparsować");
	    3.14
	}
	Ok(v) => v,
    };

    println!("{}", pi);
    println!("{X_TO_X}");
}
