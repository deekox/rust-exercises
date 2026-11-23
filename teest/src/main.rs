#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn print(&self) {
	println!("hello i am: {self:?}");
    }

    fn area(&self) -> i32 {
	self.width * self.height
    }

}

fn main() {
    let r = Rectangle {
	width: 320,
	height: 190,
    };

    r.print();
    println!("the area is {}", r.area());
}


