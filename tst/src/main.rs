fn ret_first(slice: &str) -> &str {
    if slice.len() > 0 {
	&slice[0..1]
    } else {
	&slice[..]
    }
}

fn get_arr(ar: &mut[i32]) {
    print!("ar size is {}. ar: ", ar.len());
    for &mut a in ar {
	print!("{a} ");
    }
    println!();
}

fn main() {
    let s = String::from("other string");

    let sl1 = ret_first(&s);
    let sl2 = ret_first("string one");
    println!("sl1: {sl1}\nsl2: {sl2}");

    println!("sl3: {}", &"dupa"[..1]);

    let mut a: [i32;3] = [1, 2, 3];
    get_arr(&mut a[..2]);
}

