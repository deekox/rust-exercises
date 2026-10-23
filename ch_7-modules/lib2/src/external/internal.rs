pub fn int() {
    println!("lib1::external::internal::int()");
    crate::external::ex();
    crate::run();
    crate::run_2();

//    lib2::run();
}
