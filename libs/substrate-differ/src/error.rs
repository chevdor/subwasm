use thiserror::Error;

pub type Result<T> = std::result::Result<T, SubstrateDifferError>;

#[derive(Error, Debug, Clone)]
pub enum SubstrateDifferError {
	#[error("HashComputing")]
	HashComputing(),

	#[error("Error while serializing")]
	Serialization(),

	#[error("Registry resolving error for id: `{0}`")]
	RegistryError(u32),

	#[error("Unknown")]
	Unknown(),
}

// impl<A> FromIterator for Result<A> {
//     fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
//         todo!()
//     }
// }
