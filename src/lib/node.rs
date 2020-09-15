use std::error::Error;
use std::path::Path;

use echo_lib::{Echo, ObjectId, Point, Target};

use crate::lib::core::{ChangeOrder, Feature, Node, Star};

pub fn start<P: AsRef<Path>>(data_dir: P) -> Result<impl Node, Box<dyn Error>> {
	let pending_pushes = Echo::connect("pending-pushes", data_dir.as_ref());
	Ok(LocalNode { pending_pushes })
}

struct LocalNode { pending_pushes: Echo }

impl Node for LocalNode {
	fn push(&self, orders: &Vec<ChangeOrder>) -> Result<(), Box<dyn Error>> {
		let writes = orders.iter().map(|change| match change {
			&ChangeOrder::Add(star, feature, value) => {
				let object_id = object_id(star);
				let point = echo_point(feature);
				let target = Target::String(value.to_string());
				(object_id, point, target)
			}
			&ChangeOrder::Drop(star, feature, _value) => {
				let object_id = object_id(star);
				let point = echo_point(feature);
				let target = Target::String("".to_string());
				(object_id, point, target)
			}
		}).collect::<Vec<_>>();
		self.pending_pushes.write(move |scope| {
			writes.iter().for_each(|(object_id, point, target)| {
				scope.write_object_properties(object_id, vec![(&point, target.clone())]);
			});
		})?;
		Ok(())
	}
}


fn object_id(star: &dyn Star) -> ObjectId {
	ObjectId::String(star.star_display_code().to_string())
}

fn echo_point(feature: &dyn Feature) -> Point {
	Point::String { aspect: feature.facet().to_string(), name: feature.point().to_string() }
}
