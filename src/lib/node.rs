use std::error::Error;
use std::path::Path;

use echo_lib::{Echo, Target};

use crate::lib::core::{ChangeOrder, echo_point, Node, object_id, View};

pub fn start<P: AsRef<Path>>(data_dir: P) -> Result<impl Node, Box<dyn Error>> {
	let pending_pushes = Echo::connect("pending-pushes", data_dir.as_ref());
	Ok(LocalNode { pending_pushes })
}

struct LocalNode { pending_pushes: Echo }

impl Node for LocalNode {
	fn push(&self, orders: &Vec<ChangeOrder>) {
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
		}).expect("Failed to write to pending_pushes");
	}

	fn view(&self) -> View {
		let pending_pushes = self.pending_pushes.chamber().expect("Failed to chamber pending_pushes");
		View { pending_pushes }
	}
}

