use std::str::FromStr;
use wasm_loader::NodeEndpoint;

pub fn get_chain_urls(name: &str) -> Option<Vec<NodeEndpoint>> {
	match name {
		"polkadot" | "dot" => Some(vec![
			"wss://rpc.polkadot.io:443",
			// "wss://polkadot.api.onfinality.io:443/public-ws",
			// "wss://polkadot.elara.patract.io",
		]),
		"kusama" | "ksm" => Some(vec![
			"wss://kusama-rpc.polkadot.io:443",
			"wss://kusama.api.onfinality.io:443/public-ws",
			// "wss://kusama.elara.patract.io",
		]),
		"westend" | "wnd" => Some(vec![
			"wss://westend-rpc.polkadot.io:443",
			// "wss://westend.api.onfinality.io:443/public-ws",
			// "wss://westend.elara.patract.io",
		]),
		"rococo" => Some(vec![
			"wss://rococo-rpc.polkadot.io:443",
			"wss://rococo.api.onfinality.io:443/public-ws",
			// "wss://rococo.elara.patract.io",
		]),
		"statemint" => Some(vec![
			"wss://statemint-rpc.polkadot.io:443",
			"wss://statemint.api.onfinality.io:443/public-ws",
			"wss://statemint-rpc.dwellir.com:443",
		]),
		"statemine" => Some(vec![
			"wss://statemine-rpc.polkadot.io:443",
			"wss://statemine.api.onfinality.io:443/public-ws",
			"wss://statemine-rpc.dwellir.com:443",
		]),
		"westmint" => Some(vec![
			"wss://westmint-rpc.polkadot.io:443",
			// "wss://westmint.westend.elara.patract.io",
		]),
		"karura" | "kar" => Some(vec![
			"wss://karura-rpc-0.aca-api.network:443",
			"wss://karura-rpc-1.aca-api.network:443",
			"wss://karura-rpc-2.aca-api.network:443/ws",
		]),
		"moonbase" => Some(vec!["wss://wss.api.moonbase.moonbeam.network:443"]),
		"moonriver" | "movr" => Some(vec!["wss://wss.api.moonriver.moonbeam.network:443"]),
		"moonbeam" | "glmr" => Some(vec!["wss://wss.api.moonbeam.network:443"]),
		"local" => Some(vec!["http://localhost:9933"]),
		_ => None,
	}
	.map(|s| s.into_iter().map(|s| NodeEndpoint::from_str(s).expect("Valid chain name")).collect())
}
