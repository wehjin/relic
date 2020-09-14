pub trait Star {}

pub trait StarSecret: Star {}

pub trait Feature {
	fn facet(&self) -> &str;
	fn point(&self) -> &str;
}

pub enum ChangeOrder<'a> {
	Add(&'a dyn Star, &'a dyn Feature, &'a str),
	Drop(&'a dyn Star, &'a dyn Feature, &'a str),
}

/// Top-level trait giving access to edits and reads.
pub trait Node {
	/// Publish orders to the network.
	fn publish(&self, orders: &Vec<ChangeOrder>);
}
