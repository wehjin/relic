use crate::lib::core::*;
use crate::lib::feature::StaticFeature;
use crate::lib::node;
use crate::lib::star::StaticStarSecret;

const MY_STAR_SECRET: StaticStarSecret = StaticStarSecret {};
const FEATURE_STAR_ALIAS: StaticFeature = StaticFeature { facet: "Star", point: "Alias" };

#[test]
fn it_works() {
	let node = node::start();
	node.push(&vec![
		ChangeOrder::Add(&MY_STAR_SECRET, &FEATURE_STAR_ALIAS, "Bartholomew"),
		ChangeOrder::Drop(&MY_STAR_SECRET, &FEATURE_STAR_ALIAS, "Andromeda"),
	]);
	assert_eq!(2 + 2, 4);
}
