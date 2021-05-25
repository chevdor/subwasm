use serde_json::Value;

/// Ensure that all numbers in the array are Numbers
fn all_numbers(v: &[Value]) -> bool {
	v.iter().all(|x| x.is_number())
}

/// Takes a Value. If that's an array of Numbers, return the collapsed String.
/// If not, return None.
fn replace_byte_arrays(array: &mut Value) {
	match array.as_array() {
		Some(a) if all_numbers(&a) => *array = numbers_to_string(a),
		_ => {}
	}
}

/// Collapse an array of Numbers into a String
fn numbers_to_string(v: &[Value]) -> Value {
	let bytes: Vec<u32> =
		v.iter().map(|value| serde_json::from_value(value.to_owned()).expect("Array of Number is expected")).collect();
	Value::String(bytes.iter().map(|x| format!("{:02X?}", x)).collect::<String>())
}

pub fn json_collapse_byte_arrays(json: &mut Value) {
	match json {
		Value::Array(a) if all_numbers(&a) => replace_byte_arrays(json),
		Value::Array(a) => {
			for elem in a {
				json_collapse_byte_arrays(elem);
			}
		}
		Value::Object(o) => {
			for (_k, v) in o {
				json_collapse_byte_arrays(v);
			}
		}
		_ => {}
	}
}
