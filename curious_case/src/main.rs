use rand::Rng;
//use std::cmp::Ordering;

fn main() {
    let s = "123.33";

    // Example 1
    // you need to provide type annotation here
    // but the type is BEFORE function call

    let _val: f64 = s.parse().expect("Wrong value");


    
    // Example 2
    
    let val = rand::thread_rng().gen_range(1..=100);

    // tutaj kompilator jeszcze nie wie jakiego typu jest val 
    
    println!("val: {val}");

    // tutaj też nie

    let mut val2: u8 = val;
    

    // if true {
    // 	let un: u8 = 5;
    // 	match val.cmp(&un) { // dopiero tutaj się dowiaduje, że to ma być u8!!!
    // 	    _ => {}
    // 	}
    // } else {
    // 	let int: i32 = 5;
    // 	match val.cmp(&int) { // dlatego tutaj wywala błąd bo ma i32
    // 	    _ => {}
    // 	}
    // }
}
