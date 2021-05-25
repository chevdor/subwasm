use super::raw_differ_options::RawDifferOptions;
use crate::{call_wrapper::CallWrapper, differs::json_utils::json_collapse_byte_arrays};
use log::debug;
use serde::Serialize;
use treediff::{diff, tools::Recorder};

pub struct RawDiffer<'a, T: Serialize> {
	r1: &'a T,
	r2: &'a T,
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

impl<'a, T: Serialize> RawDiffer<'a, T> {
	pub fn new(r1: &'a T, r2: &'a T) -> Self {
		Self { r1, r2 }
	}

	// TODO: return a result object instead of just printing
	/// This is a raw comparison based on the json serialization of the metadata
	pub fn compare(&self, options: RawDifferOptions) {
		let mut recorder = Recorder::default();

		let mut jsona = serde_json::value::to_value(self.r1).unwrap();
		let mut jsonb = serde_json::value::to_value(self.r2).unwrap();

		if options.collapse {
			json_collapse_byte_arrays(&mut jsona);
			json_collapse_byte_arrays(&mut jsonb);
		}

		diff(&jsona, &jsonb, &mut recorder);

		for call in &recorder.calls {
			let wrapped_call = CallWrapper(call);
			match call {
				treediff::tools::ChangeType::Removed(k, _) | treediff::tools::ChangeType::Modified(k, _, _) => {
					let doc = treediff::value::Key::String("documentation".to_string());
					if !k.contains(&doc) || k.contains(&doc) && !options.skip_doc {
						println!("{}", wrapped_call.to_string())
					}
				}
				_ => {}
			}
		}

		if options.stats {
			let mut counter = ChangeCounter::default();

			for call in &recorder.calls {
				match call {
					treediff::tools::ChangeType::Removed(_, _) => counter.inc(ChangeType::Removed),
					treediff::tools::ChangeType::Added(_, _) => counter.inc(ChangeType::Added),
					treediff::tools::ChangeType::Unchanged(_, _) => counter.inc(ChangeType::Unchanged),
					treediff::tools::ChangeType::Modified(_, _, _) => counter.inc(ChangeType::Modified),
				}
			}

			debug!("counter\t\t= {:?}", counter);
			println!(
				"unmodified\t= {:>5.02}% ({})",
				counter.percent(ChangeType::Unchanged) * 100_f64,
				counter.unchanged
			);
			println!("added\t\t= {:>5.02}% ({})", counter.percent(ChangeType::Added) * 100_f64, counter.added);
			println!("modified\t= {:>5.02}% ({})", counter.percent(ChangeType::Modified) * 100_f64, counter.modified);
			println!("removed\t\t= {:>5.02}% ({})", counter.percent(ChangeType::Removed) * 100_f64, counter.removed);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;
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
		RawDiffer::new(metadata_a, metadata_b).compare(RawDifferOptions::default());
	}

	#[test]
	fn it_compares_arrays() {
		let json1 = json!({"foo": "bar", "lst": ["a", "b", "c"], "bytes": [ 0,1,2,3]});
		let json2 = json!({"foo": "bar", "lst": ["a", "b", "c"], "bytes": [ 9,1,2,3]});
		RawDiffer::new(&json1, &json2).compare(RawDifferOptions::default());
	}

	#[test]
	fn it_compares_with_byte_arrays() {
		let mut json1 =
			json!({"foo": "bar", "lst": ["a", "b", "c"], "bytes": [ 0,255,0], "foobar": {"bar": [1,1,255]} });
		let mut json2 =
			json!({"foo": "bar", "lst": ["a", "b", "c"], "bytes": [ 2,2,255], "foobar": {"bar": [3,255,3]} });
		RawDiffer::new(&json1, &json2).compare(RawDifferOptions::default());

		json_collapse_byte_arrays(&mut json1);
		json_collapse_byte_arrays(&mut json2);
		RawDiffer::new(&json1, &json2).compare(RawDifferOptions::default());
	}
}
