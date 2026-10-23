mod libra {
    mod inner {
	pub fn fun() {
	    println!("libra::inner::fun()");
	    super::other();
	}

    }

    pub fn fun() {
	println!("libra::fun()");
	inner::fun();
    }

    fn other() {
	println!("libra::other()");
    }
}

pub fn ext() {
    println!("lib1::ext()");
    libra::fun();
}
