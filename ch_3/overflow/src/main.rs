fn main() {
    // let mut x = 127i8;
    // println!("{x}");
    // println!("{x}");


    let t1: (f32, i16, u8) = (3.1415926535, 32_627, 255);

    println!("{}, {}, {}", t1.0, t1.1, t1.2);

    let tup = (1, 'A', 3.1415_926_535_8979_323_84_626);
    let (x, y, z) = tup;

    println!("{x}, {y}, {z}");
}
