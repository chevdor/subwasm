use comparable::Comparable;
use serde::Serialize;
// use super::calls::{prelude::PalletId, *};
// use frame_metadata::PalletMetadata;
// use scale_info::form::PortableForm;
// use std::{collections::BTreeMap, fmt::Display};

/// A [ReducedPallet] could be a `Vec` or [PalletItem] but it ends
/// but providing a much more useful output after diffing when using
/// separated fields.
#[derive(Debug, PartialEq, Hash, Comparable, Serialize, Clone)]
pub struct ReducedExtrinsic {}
