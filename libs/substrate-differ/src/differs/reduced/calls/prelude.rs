pub use crate::differs::reduced::pallet_item::PalletItem;
pub use displayable_vec::*;
pub use scale_info::{form::PortableForm, Field, PortableRegistry, TypeDefVariant};

use super::displayable_vec;

pub type Documentation = Vec<String>;
pub type PalletId = u32;
pub type ExtrinsicId = u32;
pub type Value = Vec<u8>;
