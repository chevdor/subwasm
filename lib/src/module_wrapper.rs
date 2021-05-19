use frame_metadata::{v12, v13};

pub enum ModuleMetadata<'a> {
	V12(&'a v12::ModuleMetadata),
	V13(&'a v13::ModuleMetadata),
	// V14(v14::ModuleMetadata),
}
pub struct ModuleWrapper<'a>(pub ModuleMetadata<'a>);

impl<'a> ModuleWrapper<'a> {
	pub fn display_module(&self) {
		println!("WE DISP A MODULE HERE");

		match &self.0 {
			ModuleMetadata::V12(v12) => println!(" - {:02}: {:?}", v12.index, v12.name),
			_ => panic!("Runtime not supported"),
		};
	}
}
