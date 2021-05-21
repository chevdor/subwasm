use std::fmt::Display;
use treediff::tools::ChangeType;

pub struct CallWrapper<'a, K, V: 'a>(pub &'a ChangeType<'a, K, V>);

impl<'a, K: Display, V: Display + 'a> Display for CallWrapper<'a, K, V> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.0 {
			ChangeType::Modified(keys, v_before, v_after) => {
				let mut res = String::new();
				for key in keys {
					res.push_str(&format!("{}.", key));
				}

				write!(f, "🛠  {:<50} {:>20} --> {}", res, v_before.to_string(), v_after.to_string())
			}
			ChangeType::Removed(keys, val) => {
				let mut res = String::new();
				for key in keys {
					res.push_str(&format!("{}.", key));
				}
				write!(f, "🗑  {:<50} {:>20}", res, val.to_string())
			}

			ChangeType::Unchanged(_, _) => write!(f, "Unchanged"),
			ChangeType::Added(_, _) => write!(f, "Added"),
		}
	}
}
