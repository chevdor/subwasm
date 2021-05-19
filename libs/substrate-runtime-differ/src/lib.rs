pub struct MetadataDiffer {}

// struct MedatataDiff;

impl MetadataDiffer {
	pub fn new(_m1: &str, _m2: &str) -> Self {
		println!("++");
		// todo!();
		Self {}
	}

	pub fn diff() {
		todo!();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_constructs() {
		// let m1 = RuntimeMetadataPrefixed::from("");
		let _md = MetadataDiffer::new("a", "b");
		assert_eq!(2 + 2, 4);
	}
}
