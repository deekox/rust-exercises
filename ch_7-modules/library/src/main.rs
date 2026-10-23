//use library::outer;

fn main() {
    let l = library::outer::add(1, 2);
    println!("l: {l}");

    let r = library::outer::sub(3, 1);
    println!("r: {r}");
    
}
    
