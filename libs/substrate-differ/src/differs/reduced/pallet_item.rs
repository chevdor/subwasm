use super::pallet_data::PalletData;

#[derive(Debug, PartialEq)]
pub enum PalletItem {
	Call(PalletData),
	Event(PalletData),
	Error(PalletData),
	Storage(PalletData),
	Constant(PalletData),
}
