mod ext {
    mod int {
	pub fn add(left: u64, right: u64) -> u64 {
	    left + right
	}
    }
    pub use int::add;
}

pub mod public {
    pub use crate::ext::add;
}
