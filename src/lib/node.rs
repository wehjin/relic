use crate::lib::core::{ChangeOrder, Node};

pub fn start() -> impl Node {
	LocalNode {}
}

struct LocalNode {}

impl Node for LocalNode {
	fn push(&self, orders: &Vec<ChangeOrder>) {}
}

