use std::fmt::Display;
use std::fmt::Write as _;
use treediff::tools::ChangeType;

pub struct CallWrapper<'a, K, V: 'a>(pub &'a ChangeType<'a, K, V>);

impl<'a, K: Display, V: Display + 'a> Display for CallWrapper<'a, K, V> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.0 {
			ChangeType::Modified(keys, v_before, v_after) => {
				let mut res = String::new();
				for key in keys {
					let _ = write!(res, "{key}.");
				}

				write!(f, "🛠  {res:<50} {v_before:>20} --> {v_after}")
			}
			ChangeType::Removed(keys, val) => {
				let mut res = String::new();
				for key in keys {
					let _ = write!(res, "{key}.");
				}
				write!(f, "🗑  {res:<50} {val:>20}")
			}

			ChangeType::Unchanged(_, _) => write!(f, "Unchanged"),
			ChangeType::Added(_, _) => write!(f, "Added"),
		}
	}
}
