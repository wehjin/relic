pub trait Star {}

pub trait StarSecret {
	fn star(&self) -> dyn Star;
}

pub trait Feature<V: ValType> {
	fn facet(&self) -> &str;
	fn point(&self) -> &str;
	fn val_type(&self) -> &V;
}

pub trait ValType {}

// pub enum EditOrder<'a, V: ValType> {
// 	Set(&'a dyn Star, &'a dyn Feature<V>, &'a V),
// 	Clr(&'a dyn Star, &'a dyn Feature<V>, &'a V),
// }
