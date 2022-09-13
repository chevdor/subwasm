use crate::differs::reduced::diff_analyzer::Compatible;

use super::call::*;
use super::constant::*;

impl Compatible for CallChange {
	fn compatible(&self) -> bool {
		println!("self = {:#?}", self);

		todo!()
	}
}

impl Compatible for ConstantChange {
	fn compatible(&self) -> bool {
		match self {
			ConstantChange::Name(_n) => false,
			ConstantChange::Value(_v) => true,
		}
	}
}
