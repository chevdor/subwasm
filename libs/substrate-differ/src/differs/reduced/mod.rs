#![allow(clippy::derive_partial_eq_without_eq)]

pub mod calls;
pub mod signature;
pub mod changed_wapper;
pub mod diff_analyzer;
pub mod reduced_diff_result;
pub mod reduced_differ;

pub mod pallet_data;
pub mod pallet_item;
pub mod reduced_pallet;
pub mod reduced_pallet_change;
pub mod reduced_pallet_change_wrapper;

pub mod reduced_runtime;
pub mod reduced_runtime_change_wrapper;


#[cfg(feature = "v13")]
pub mod v13;
#[cfg(feature = "v14")]
pub mod v14;

enum ComparisonSide {
	Left,
	Right,
}
