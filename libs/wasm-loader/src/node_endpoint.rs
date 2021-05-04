/// A [`NodeEndpoint`] can be either `Http` or `WebSocket`.
#[derive(Debug, Clone, PartialEq)]
pub enum NodeEndpoint {
	Http(String),
	WebSocket(String),
}
