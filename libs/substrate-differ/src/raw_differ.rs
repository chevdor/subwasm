use frame_metadata::RuntimeMetadata;
// use rustc_serialize::json::Json;
use treediff::{diff, tools::Recorder};

use crate::call_wrapper::CallWrapper;

pub struct MetadataRawDiffer<'a> {
	r1: &'a RuntimeMetadata,
	r2: &'a RuntimeMetadata,
}

enum ChangeType {
	Removed,
	Added,
	Unchanged,
	Modified,
}

#[derive(Debug, Default)]
struct ChangeCounter {
	removed: u64,
	added: u64,
	unchanged: u64,
	modified: u64,
}

impl ChangeCounter {
	pub fn inc(&mut self, ty: ChangeType) {
		match ty {
			ChangeType::Removed => self.removed += 1,
			ChangeType::Added => self.added += 1,
			ChangeType::Unchanged => self.unchanged += 1,
			ChangeType::Modified => self.modified += 1,
		}
	}

	pub fn percent(&self, ty: ChangeType) -> f64 {
		let sum = (self.unchanged + self.modified + self.removed + self.added) as f64;
		let relevant = match ty {
			ChangeType::Removed => self.removed as f64,
			ChangeType::Added => self.added as f64,
			ChangeType::Unchanged => self.unchanged as f64,
			ChangeType::Modified => self.modified as f64,
		};
		relevant / sum
	}
}

impl<'a> MetadataRawDiffer<'a> {
	pub fn new(r1: &'a RuntimeMetadata, r2: &'a RuntimeMetadata) -> Self {
		Self { r1, r2 }
	}

	/// This is a raw comparison based on the json serialization of the metadata
	pub fn compare(&self) {
		let mut recorder = Recorder::default();

		let jsona = serde_json::value::to_value(self.r1).unwrap();
		let jsonb = serde_json::value::to_value(self.r2).unwrap();
		diff(&jsona, &jsonb, &mut recorder);

		// let v1: Json =
		// 	r#"{"one": 1, "foo": "bar", "bar": { "a": 1}, "x1": { "x2": { "x3": 1}}, "x2": { "x2": { "x3": 1}}}"#
		// 		.parse()
		// 		.unwrap();
		// let v2: Json = r#"{"one": 1, "foo": "bary", "bar": { "a": 2}, "x2": { "x2": { "x3": 3}}}"#.parse().unwrap();
		// diff(&v1, &v2, &mut recorder);

		let mut counter = ChangeCounter::default();
		// println!("recorder = {:#?}", recorder);

		for call in &recorder.calls {
			match call {
				treediff::tools::ChangeType::Removed(_, _) => counter.inc(ChangeType::Removed),
				treediff::tools::ChangeType::Added(_, _) => counter.inc(ChangeType::Added),
				treediff::tools::ChangeType::Unchanged(_, _) => counter.inc(ChangeType::Unchanged),
				treediff::tools::ChangeType::Modified(_, _, _) => counter.inc(ChangeType::Modified),
			}
		}

		for call in &recorder.calls {
			let wrapped_call = CallWrapper(call);
			match call {
				treediff::tools::ChangeType::Removed(k, _) => {
					let doc = treediff::value::Key::String("documentation".to_string());
					// skipping doc
					if !k.contains(&doc) {
						println!("{}", wrapped_call.to_string())
					}
				}

				treediff::tools::ChangeType::Modified(k, _, _) => {
					let doc = treediff::value::Key::String("documentation".to_string());
					// skipping doc
					if !k.contains(&doc) {
						println!("{}", wrapped_call.to_string())
					}
				}
				_ => {}
			}
		}

		println!("counter\t\t= {:?}", counter);
		println!("%unmodified\t= {:>5.02}% ({})", counter.percent(ChangeType::Unchanged) * 100_f64, counter.unchanged);
		println!("%added\t\t= {:>5.02}% ({})", counter.percent(ChangeType::Added) * 100_f64, counter.added);
		println!("%modified\t= {:>5.02}% ({})", counter.percent(ChangeType::Modified) * 100_f64, counter.modified);
		println!("%removed\t= {:>5.02}% ({})", counter.percent(ChangeType::Removed) * 100_f64, counter.removed);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::path::PathBuf;
	use wasm_loader::Source;
	use wasm_testbed::WasmTestBed;

	#[test]
	#[ignore = "local data"]
	fn it_constructs() {
		const RTM1: &str = "../../data/kusama/kusama-2030.wasm";
		const RTM2: &str = "../../data/kusama/kusama_runtime-v9000.compact.wasm";

		let runtime_a = WasmTestBed::new(&Source::File(PathBuf::from(RTM1))).unwrap();
		let runtime_b = WasmTestBed::new(&Source::File(PathBuf::from(RTM2))).unwrap();
		let metadata_a = runtime_a.metadata();
		let metadata_b = runtime_b.metadata();
		let md = MetadataRawDiffer::new(metadata_a, metadata_b);
		md.compare();
	}
}
