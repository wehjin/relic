use crate::lib::core::{Star, StarSecret};

pub struct StaticStarSecret {}

impl StarSecret for StaticStarSecret {}

impl Star for StaticStarSecret {}