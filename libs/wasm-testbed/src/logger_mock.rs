use sp_wasm_interface::ValueType::I32;
use sp_wasm_interface::{Function, FunctionContext, Signature, Value};

/// Newer runtimes require the implementation of a HostFunction called
/// `ext_logging_max_level_version_1`. This mock provides a version doing
/// nothing but allowing to decode newer runtimes.
pub struct LoggerMock;

impl Function for LoggerMock {
	fn name(&self) -> &str {
		"ext_logging_max_level_version_1"
	}

	fn signature(&self) -> Signature {
		// Signature::new(vec![], None)
		Signature::new(vec![], Some(I32))
	}

	fn execute(
		&self,
		_context: &mut dyn FunctionContext,
		_args: &mut dyn Iterator<Item = Value>,
	) -> sp_wasm_interface::Result<Option<Value>> {
		Ok(Some(Value::I32(0)))
	}
}
