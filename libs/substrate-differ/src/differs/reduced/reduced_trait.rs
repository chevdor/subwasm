use super::{reduced_pallet::ReducedPallet, reduced_runtime::ReducedRuntime};

/// Marker trait
pub trait Reduced {}
pub trait ReducedRtm {}

impl Reduced for ReducedRuntime {}
impl ReducedRtm for ReducedRuntime {}
impl Reduced for ReducedPallet {}
