use crate::lib::core::Feature;

/// A feature based on static strings.
pub struct StaticFeature {
	pub facet: &'static str,
	pub point: &'static str,
}

impl Feature for StaticFeature {
	fn facet(&self) -> &str { self.facet }
	fn point(&self) -> &str { self.point }
}
