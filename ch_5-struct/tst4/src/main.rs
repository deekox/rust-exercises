#[derive(Debug)]
struct Rect {
    width: i32,
    height: i32
}

impl Rect {
    fn area(&self) -> i32 {
	self.width * self.height
    }
    fn fits(&self, other: &Self) -> bool {
	self.height >= other.height &&
	    self.width >= other.width ||
	    self.height >= other.width &&
	    self.width >= other.height
    }

    fn new(width: i32, height: i32) -> Self {
	Self{
	    width,
	    height,
	}
    }

    fn assign_to_self(&mut self, other: Self) {
	*self = other;
    }

    // fn move_out_of_self(&mut self) {
    // 	let r: Rect = *self;
    // }

    
    fn max(self, other: Rect) -> Rect {
        Rect { 
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    // fn set_to_max(&mut self, other: Self) {
    // 	let max = Self::max(*self, other); // L2
    // 	*self = max;
    // }
}

struct Moja {
    tab: [i32; 100],
}

fn main() {
    let mut tab1 = Moja { tab: [0; 100] };
    let tab2 = Moja { tab: [3; 100] };
    tab1 = tab2;
//    let x = tab2.tab[2];
    
    
    let mut r1 = Rect::new(10, 15);
    let r2 = Rect::new(15, 10);

    let mref = &mut r1;
    *mref = r2;  // this is move of r2 to r1
    // note that borrowing does not mean that borrowed object
    // will be the same (i.e. the owner will point to the same object as before borrow)
    // (( or ownership does not imply that owner points to the same data ))
    // borrowing guarantees that the ownership will not change
    // (i.e. some data will be destroyed when owner goes out of scope)

    
    
    
    
    
    
    //Rect::assign_to_self(&mut r1, r2);
    
    
    let b = true && false || true;
    println!("true && false || true   == {b}");


    

    let mut rs: &mut String = &mut String::from("hello");
//    let rx = *rs;
}
