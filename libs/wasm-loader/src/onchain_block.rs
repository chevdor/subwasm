use crate::NodeEndpoint;

pub type BlockRef = String; // TODO: to change

/// This structure points to a node url and an optional block reference.
#[derive(Debug, Clone)]
pub struct OnchainBlock {
	pub url: NodeEndpoint,
	pub block_ref: Option<BlockRef>,
}
