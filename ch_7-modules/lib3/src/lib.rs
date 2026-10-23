pub mod ext {
    pub mod mid {
	pub mod int {
	    pub fn fun() {
		println!("ext::mid::int::fun()");
		crate::ext::fun();
		super::fun();
		super::super::fun();
	    }
	}
	fn fun() {
	    println!("ext::mid::fun()");
	}

    }
    fn fun() {
    	println!("ext::fun()");
    }
}
