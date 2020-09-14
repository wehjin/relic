use crate::lib::core::{ChangeOrder, Node};

pub fn start() -> impl Node {
	LocalNode {}
}

struct LocalNode {}

impl Node for LocalNode {
	fn publish(&self, orders: &Vec<ChangeOrder>) {}
}

