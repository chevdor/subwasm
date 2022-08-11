pub mod call;
pub mod diff_result;
pub mod diff_trait;
pub mod pallet_data;
pub mod pallet_item;
pub mod reduced_differ;
pub mod reduced_pallet;
pub mod reduced_runtime;
pub mod signature;

/// This is the pallet name and the pallet index.
pub type PalletId = (String, Index);

#[cfg(feature = "v13")]
pub mod v13;
#[cfg(feature = "v14")]
pub mod v14;
