use echo_lib::{Chamber, ObjectId, Point};

pub trait Star {
	fn star_display_code(&self) -> &str;
}

pub trait StarSecret: Star {}

pub trait Feature {
	fn facet(&self) -> &str;
	fn point(&self) -> &str;
}

pub enum ChangeOrder<'a> {
	Add(&'a dyn Star, &'a dyn Feature, &'a str),
	Drop(&'a dyn Star, &'a dyn Feature, &'a str),
}

/// Entry-point into the network.
pub trait Node {
	/// Publish orders to the network.
	fn push(&self, orders: &Vec<ChangeOrder>);

	/// Acquire the latest view of the network.
	fn view(&self) -> View;
}

/// Provides a view of data in the network.
pub struct View {
	pub(crate) pending_pushes: Chamber
}

impl View {
	/// Read a value from unconfirmed values.
	pub fn unconfirmed_value(&self, star: &impl Star, feature: &impl Feature) -> Option<String> {
		let object_id = object_id(star);
		let point = echo_point(feature);
		self.pending_pushes.target_at_object_point_or_none(&object_id, &point).map(|it| it.to_string())
	}
}

pub(crate) fn object_id(star: &dyn Star) -> ObjectId {
	ObjectId::String(star.star_display_code().to_string())
}

pub(crate) fn echo_point(feature: &dyn Feature) -> Point {
	Point::String { aspect: feature.facet().to_string(), name: feature.point().to_string() }
}
