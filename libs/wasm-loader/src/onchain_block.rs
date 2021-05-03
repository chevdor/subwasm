use crate::NodeEndpoint;

// TODO: to change
pub type BlockRef = String;

/// This structure points to a node url and an optional block reference.
#[derive(Debug, Clone)]
pub struct OnchainBlock {
	pub url: NodeEndpoint,
	pub block_ref: Option<BlockRef>,
}
