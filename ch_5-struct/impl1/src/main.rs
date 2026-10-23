#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32 {
	self.width * self.height
    }

    fn sum_of_area(&self, other: &Self) -> u32 {
	self.area() + other.area()
    }
}



fn main() {
    let r = Rect{width: 3, height: 4};
    let r2 = Rect{..r};
    
    println!("{:?} area is: {}", r, Rect::area(&r));
    println!("sum_of_area is: {}", r.sum_of_area(&r2));
    
}
