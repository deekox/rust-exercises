// note: remove Copy from attributes to fail the compilation 

#[derive(Debug, Copy, Clone)]
struct Person {
    age: i32,
}

impl Person {
    fn make_max(&mut self, other: Self) {
	*self = Self::max(*self, other);
    }

    fn max(l: Self, r: Self) -> Self {
	Self {
	    age: l.age.max(r.age)
	}
    }
}


fn main() {
    let mut p1 = Person {
	age: 67,
    };
    let p2 = Person {
	age: 50,
    };

    let a = 3;
    println!("{p1:?}");
    println!("{p2:?}");

    Person::make_max(&mut p1, p2);
    println!("{p1:?}");


    let v1 = vec![1];
    println!("v1: {:p}", &raw const v1); 
    let v2 = v1;

    println!("v2: {:p}", &raw const v2); 
}
