use std::str::FromStr;
use wasm_loader::NodeEndpoint;

pub fn get_chain_urls(name: &str) -> Option<Vec<NodeEndpoint>> {
	match name {
		"polkadot" | "dot" => Some(vec![
			"wss://rpc.polkadot.io",
			"wss://polkadot.api.onfinality.io/public-ws",
			// "wss://polkadot.elara.patract.io",
		]),
		"kusama" | "ksm" => Some(vec![
			"wss://kusama-rpc.polkadot.io",
			"wss://kusama.api.onfinality.io/public-ws",
			// "wss://kusama.elara.patract.io",
		]),
		"westend" | "wnd" => Some(vec![
			"wss://westend-rpc.polkadot.io",
			"wss://westend.api.onfinality.io/public-ws",
			// "wss://westend.elara.patract.io",
		]),
		"rococo" => Some(vec![
			"wss://rococo-rpc.polkadot.io",
			"wss://rococo.api.onfinality.io/public-ws",
			// "wss://rococo.elara.patract.io",
		]),
		"statemint" => Some(vec!["wss://polkadot-statemint-rpc.paritytech.net"]),
		"statemine" => Some(vec![
			"wss://kusama-statemine-rpc.paritytech.net",
			"wss://statemine.api.onfinality.io/public-ws",
			// "wss://statemine.kusama.elara.patract.io",
		]),
		"westmint" => Some(vec![
			"wss://westmint-rpc.polkadot.io",
			// "wss://westmint.westend.elara.patract.io",
		]),
		"karura" | "kar" => Some(vec![
			"wss://karura-rpc-0.aca-api.network",
			"wss://karura-rpc-1.aca-api.network",
			"wss://karura-rpc-2.aca-api.network/ws",
		]),
		"local" => Some(vec!["http://localhost:9933"]),
		_ => None,
	}
	.map(|s| s.into_iter().map(|s| NodeEndpoint::from_str(s).expect("Valid chain name")).collect())
}
