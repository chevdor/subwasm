use log::info;
use num_format::{Locale, ToFormattedString};
use wasm_testbed::WasmTestBed;

pub struct RuntimeSummaryDiffer<'a> {
	r1: &'a WasmTestBed,
	r2: &'a WasmTestBed,
}

impl<'a> RuntimeSummaryDiffer<'a> {
	pub fn new(r1: &'a WasmTestBed, r2: &'a WasmTestBed) -> Self {
		Self { r1, r2 }
	}

	pub fn compare(&self) {
		let size = |x| -> (f32, usize) { (x as f32 / 1024.0 / 1024.0, x) };

		// RUNTIME SIZE
		let size_a = self.r1.size();
		let size_b = self.r2.size();

		info!("Checking runtime sizes:");
		if size_a == size_b {
			println!(
				"  ✅  Both size are identical: {:.3?} MB ({} bytes)",
				size(size_a).0,
				size(size_a).1.to_formatted_string(&Locale::en)
			);
		} else {
			println!("  🅰️  {:.3?} MB ({} bytes)", size(size_a).0, size(size_a).1.to_formatted_string(&Locale::en));
			println!("  🅱️  {:.3?} MB ({} bytes)", size(size_b).0, size(size_b).1.to_formatted_string(&Locale::en));
		}

		// METADATA VERSIONS
		let metadata_a_version = self.r1.metadata_version();
		let metadata_b_version = self.r2.metadata_version();
		println!("Checking metadata versions:");
		if metadata_a_version == metadata_b_version {
			println!("  ✅ Both metadata versions are identical: V{:?}", metadata_a_version);
		} else {
			println!("Found different metadata versions:");
			println!("  🅰️  V{:?}", metadata_a_version);
			println!("  🅱️  V{:?}", metadata_b_version);
		}

		// CORE VERSIONS
		println!("Checking core versions:");
		let version_a = self.r1.core_version();
		let version_b = self.r2.core_version();

		if version_a == version_b {
			print!("  ✅  The 2 core versions are identical: ");
			println!("{}", version_a);
		} else {
			println!("  ❌ The 2 core versions are different: ");
			// println!("{:#?}", version_a);
			println!("  🅰️  {}", version_a);
			// println!("{:#?}", version_b);
			println!("  🅱️  {}", version_b);
		}

		println!("Checking runtime metadata:");
		let metadata_a = self.r1.metadata();
		let metadata_b = self.r2.metadata();

		if metadata_a == metadata_b {
			// println!("  {}", version_a);
			println!("  ✅  The metadata are identical");
		} else {
			println!("  ❌  The metadata are different");
		}
	}
}
