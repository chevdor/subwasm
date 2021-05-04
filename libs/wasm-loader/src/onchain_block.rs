use crate::NodeEndpoint;

pub type BlockRef = String; // TODO: to change

/// This structure points to a node url and an optional block reference.
#[derive(Debug, Clone, PartialEq)]
pub struct OnchainBlock {
	pub endpoint: NodeEndpoint,
	pub block_ref: Option<BlockRef>,
}
