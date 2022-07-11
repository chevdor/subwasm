use std::fmt;

#[derive(Debug, Clone)]
pub enum WasmLoaderError {
	EndpointParsing(String),
	OnchainBlockParsing(String),
	NotSupported(String),
	HttpClient(),
	WsClient(),
}

impl fmt::Display for WasmLoaderError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			WasmLoaderError::EndpointParsing(s) | WasmLoaderError::OnchainBlockParsing(s) => {
				write!(f, "Parsing issue: {:?}", s)
			}

			WasmLoaderError::NotSupported(s) => write!(f, "Unsupported: {:?}", s),
			WasmLoaderError::HttpClient() => write!(f, "HttpClient Error"),
			WasmLoaderError::WsClient() => write!(f, "WsClient Error"),
		}
	}
}

impl std::error::Error for WasmLoaderError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		None
	}

	fn description(&self) -> &str {
		"description() is deprecated; use Display"
	}

	fn cause(&self) -> Option<&dyn std::error::Error> {
		self.source()
	}
}
