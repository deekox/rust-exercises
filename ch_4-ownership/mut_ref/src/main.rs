fn main() {
    let mut v = vec![1, 2, 3];
    let vr: &mut i32 = &mut v[1];
    println!("vr{}", vr);

    *vr = 4;
    println!("vr{}", *vr);

    for i in v {
	print!("{} ", i);
    }
    println!();
}
