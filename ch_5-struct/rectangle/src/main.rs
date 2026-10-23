use std::fmt;

struct Rect {
    width: i32,
    height: i32,
}

fn std:fmt::Display(rec: &Rect) -> String {
    String::from("adsf")
}
    

fn area(rect: &Rect) -> i32 {
    rect.width * rect.height
}


fn main() {
    let my = Rect {
	width: 12,
	height: 34,
    };
    
    let ar = area(&my);
    println!("w: {}, h: {}, area: {}", my.width, my.height, ar);
    println!("{my}");
}
