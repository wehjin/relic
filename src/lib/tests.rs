use crate::lib::core::*;
use crate::lib::feature::StaticFeature;
use crate::lib::node;
use crate::lib::star::StaticStarSecret;

const MY_STAR_SECRET: StaticStarSecret = StaticStarSecret {};
const FE_STAR_ALIAS: StaticFeature = StaticFeature { facet: "Star", point: "Alias" };

#[test]
fn it_works() {
	let relic = node::start();
	relic.publish(&vec![
		ChangeOrder::Add(&MY_STAR_SECRET, &FE_STAR_ALIAS, "Bartholomew"),
		ChangeOrder::Drop(&MY_STAR_SECRET, &FE_STAR_ALIAS, "Andromeda"),
	]);
	assert_eq!(2 + 2, 4);
}
