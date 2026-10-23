pub mod external;

fn run() {
    println!("lib1::run()");
}

pub fn run_2() {
    println!("lib1::run_2()");

    crate::run();
}



