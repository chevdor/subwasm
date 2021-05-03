/// A [`NodeEndpoint`] can be either `Http` or `WebSocket`.
#[derive(Debug, Clone)]
pub enum NodeEndpoint {
	Http(String),
	WebSocket(String),
}
