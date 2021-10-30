use super::{raw_differ_options::RawDifferOptions, skip_counter::SkipCounter};
use crate::{
	call_wrapper::CallWrapper,
	differs::{
		change_counter::{ChangeCounter, ChangeType},
		json_utils::json_collapse_byte_arrays,
	},
};
use log::debug;
use serde::Serialize;
use treediff::{diff, tools::Recorder};

pub struct RawDiffer<'a, T: Serialize> {
	r1: &'a T,
	r2: &'a T,
}

impl<'a, T: Serialize> RawDiffer<'a, T> {
	pub fn new(r1: &'a T, r2: &'a T) -> Self {
		Self { r1, r2 }
	}

	// TODO: return a result object instead of just printing
	/// This is a raw comparison based on the json serialization of the metadata
	pub fn compare(&self, options: RawDifferOptions) {
		let mut recorder = Recorder::default();
		let mut skip_counter = SkipCounter::default();

		let mut jsona = serde_json::value::to_value(self.r1).unwrap();
		let mut jsonb = serde_json::value::to_value(self.r2).unwrap();

		if options.collapse {
			json_collapse_byte_arrays(&mut jsona);
			json_collapse_byte_arrays(&mut jsonb);
		}

		if options.ignore_version {
			let va = jsona.as_object().expect("Shoud be a json object").iter().next().expect("Should have a version").0;
			let vb = jsonb.as_object().expect("Shoud be a json object").iter().next().expect("Should have a version").0;
			println!("Comparing {} with {}", va, vb);

			diff(&jsona[va], &jsonb[vb], &mut recorder);
		} else {
			diff(&jsona, &jsonb, &mut recorder);
		}

		for call in &recorder.calls {
			let wrapped_call = CallWrapper(call);
			match call {
				treediff::tools::ChangeType::Removed(k, val) | treediff::tools::ChangeType::Modified(k, val, _) => {
					let doc = treediff::value::Key::String("documentation".to_string());
					let value = treediff::value::Key::String("value".to_string());
					let default = treediff::value::Key::String("default".to_string());

					if k.contains(&doc) && options.skip_doc {
						skip_counter.documentation += 1;
					} else if (k.contains(&value) && (val.is_array() || options.collapse) && options.skip_bytes)
						|| (k.contains(&default) && (val.is_array() || options.collapse) && options.skip_bytes)
					{
						skip_counter.bytes += 1;
					} else {
						println!("{}", wrapped_call)
					}

					// if !k.contains(&doc) || k.contains(&doc) && !options.skip_doc {
					// 	println!("{}", wrapped_call.to_string())
					// } else {
					// 	skip_counter.documentation += 1;
					// }

					// if (!k.contains(&value) || k.contains(&value) && val.is_array() && !options.skip_bytes)
					// 	&& (!k.contains(&default) || k.contains(&default) && val.is_array() && !options.skip_bytes)
					// {
					// 	println!("{}", wrapped_call.to_string())
					// } else {
					// 	skip_counter.bytes += 1;
					// }
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
			counter.print();

			skip_counter.print();
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
		const RTM1: &str = "../../data/kusama/V12/kusama-2030.wasm";
		const RTM2: &str = "../../data/kusama/V12/kusama-9000.compact.wasm";

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
