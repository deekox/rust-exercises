fn main() {
    // let mut v = vec![1, 2, 3];
    // let mut num = &v;
    // num = &vec![4, 5, 6];

    // let mut_num: &mut i32 = &mut v[1];
    // *mut_num = 32;

    // for i in &v {
    // 	print!("{i} ");
    // }
    // println!();

    // let mut mm_num: &mut i32 = &mut v[2];
    // *mm_num = 33;
    // mm_num = &mut v[0];
    // *mm_num = -33;

    // for i in &v {
    // 	print!("{i} ");
    // }
    // println!();

    // let m_num : &mut i32 = &mut v[1];
    // *m_num = 3;

    // let v2 = vec![6, 7, 8];
    // let m_n: &mut i32 = &mut v2[1];
    let mut v = vec![1, 2, 3];
    let ri: &i32 = &v[2];
    v.push(3);
    println!("{}", *ri);
}
