fn main() {

   //   let b = Box::new(0);
   // let b2 = b;
   //  println!("{}", b);

    let v = vec![1, 2, 3];
    let mut vm = v;
    println!("{}", v[1]);

//    this makes compilation error    
//    let x: String = String::from("dupa");
//    x = String::from("asfd");


    
    // scalar
    // 1. i32
    let a = 33;
    let mut b = a;
    b += 55;
    println!("{a} {b}");


    // 2. &str
    let s1: &str = "Hello";
    let mut s2: &str = s1;
    s2 = "World";
    println!("{s1} {s2}");


    // compound
    // 1. array
    let ar1 = [1, 2, 3];
    let mut ar2: [i32; 3] = ar1;

    ar2[1] += 100;
    ar2 = [100; 3];
    
    println!("{:?}", ar1);
    println!("{:?}", ar2);

    // 2. tuple
    let t1 = (10, "Hey");
    let mut t2 = t1;

    t2.0 = t1.0 + 100;
    t2.1 = "OJOJ";
    println!("{:?}", t1);
    println!("{:?}", t2);

    let s1 = String::from("Hello");
    let mut s2 = s1;
    fun(s2);
    //    println!("{s1} {s2}");


    let s3 = String::from("World");
    fun(s3);
//    println!("{}", s3);
}

fn fun(mut s: String) {
    s += "!!!";
    println!("fun() {s}");
}
