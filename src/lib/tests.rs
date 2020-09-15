extern crate rand;

use std::error::Error;

use crate::lib::core::*;
use crate::lib::feature::StaticFeature;
use crate::lib::node;
use crate::lib::star::StaticStarSecret;

const MY_STAR_SECRET: StaticStarSecret = StaticStarSecret {};
const FEATURE_STAR_ALIAS: StaticFeature = StaticFeature { facet: "Star", point: "Alias" };

#[test]
fn it_works() -> Result<(), Box<dyn Error>> {
	let data_dir = tools::unique_tmp_dir("relic-tests-it-works")?;
	let node = node::start(&data_dir)?;
	node.push(&vec![
		ChangeOrder::Add(&MY_STAR_SECRET, &FEATURE_STAR_ALIAS, "Bartholomew"),
		ChangeOrder::Drop(&MY_STAR_SECRET, &FEATURE_STAR_ALIAS, "Andromeda"),
	])?;
	assert_eq!(2 + 2, 4);
	Ok(())
}

mod tools {
	use std::error::Error;
	use std::path::PathBuf;

	pub fn unique_tmp_dir(prefix: &str) -> Result<PathBuf, Box<dyn Error>> {
		let mut path = std::env::temp_dir();
		path.push(format!("{}-{}", prefix, rand::random::<u32>()));
		std::fs::create_dir_all(&path)?;
		Ok(path)
	}
}