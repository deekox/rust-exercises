fn main() {
    let mut x = 3;
    let mut y: &mut i32 = &mut x;
    *y += 3;
    println!("{}", y);
    x += 4;
    println!("{}", x);
    
}

