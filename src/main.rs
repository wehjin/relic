extern crate yui;

use std::error::Error;

use yui::prelude::*;
use yui::yard::{ButtonState, Pressable};

struct MainSpark {}

impl Spark for MainSpark {
	type State = ();
	type Action = ();
	type Report = ();

	fn create(&self, _ctx: &Create<Self::Action, Self::Report>) -> Self::State { () }

	fn flow(&self, _action: Self::Action, _ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		AfterFlow::Close(None)
	}


	fn render(_state: &Self::State, link: &Link<Self::Action>) -> Option<ArcYard> {
		let section_list = yard::list(200, 0, vec![
			(3, yard::label("Settings", StrokeColor::BodyOnBackground, Cling::Center).pressable(|_| {}))
		]);
		let close = yard::button("Close", ButtonState::enabled(link.callback(|_| ())));
		let side = section_list.pack_bottom(3, close.pad(1));
		Some(side)
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	yui::main(MainSpark {})?;
	Ok(())
}
