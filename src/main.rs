extern crate echo_lib;
extern crate yui;

use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};

use yui::prelude::*;
use yui::SenderLink;
use yui::yard::{ButtonState, Pressable};

pub use crate::lib::node;
pub use crate::lib::prelude;
use crate::YardId::{EditList, FacetEdit, PointEdit, ValueEdit};

mod lib;

struct MainSpark {}

#[derive(Clone)]
struct MainState {
	facet: StringEdit,
	point: StringEdit,
	value: StringEdit,
	active_index: usize,
}

enum MainAction {
	Close,
	Facet(StringEditAction),
	Point(StringEditAction),
	Value(StringEditAction),
}

impl Spark for MainSpark {
	type State = MainState;
	type Action = MainAction;
	type Report = ();

	fn create(&self, _ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		MainState {
			facet: StringEdit::empty(ValidIf::NotEmpty),
			point: StringEdit::empty(ValidIf::NotEmpty),
			value: StringEdit::empty(ValidIf::NotEmpty),
			active_index: 0,
		}
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			MainAction::Close => AfterFlow::Close(None),
			MainAction::Facet(edit_action) => {
				let mut state = ctx.state().clone();
				state.facet = state.facet.edit(edit_action);
				state.active_index = 0;
				AfterFlow::Revise(state)
			}
			MainAction::Point(edit_action) => {
				let mut state = ctx.state().clone();
				state.point = state.point.edit(edit_action);
				state.active_index = 1;
				AfterFlow::Revise(state)
			}
			MainAction::Value(edit_action) => {
				let mut state = ctx.state().clone();
				state.value = state.value.edit(edit_action);
				state.active_index = 2;
				AfterFlow::Revise(state)
			}
		}
	}


	fn render(state: &Self::State, link: &SenderLink<Self::Action>) -> Option<ArcYard> {
		let section_names = yard::trellis(3, 0, Cling::Left, vec![
			yard::label("Editor", StrokeColor::BodyOnBackground, Cling::Right).pad_cols(1).pressable(SenderLink::ignore())
		]);
		let close = yard::button("Close", ButtonState::enabled(link.clone().map(|_| MainAction::Close)));
		let sidebar = section_names.pack_top(3, close.pad(1));

		let facet_edit = yard::textfield(FacetEdit.into(), "Facet", state.facet.clone(), link.clone().map(MainAction::Facet));
		let point_edit = yard::textfield(PointEdit.into(), "Point", state.point.clone(), link.clone().map(MainAction::Point));
		let value_edit = yard::textfield(ValueEdit.into(), "Value", state.value.clone(), link.clone().map(MainAction::Value));
		let edits = yard::list(EditList.into(), state.active_index, vec![
			(4, facet_edit.confine_height(3, Cling::Top)),
			(4, point_edit.confine_height(3, Cling::Top)),
			(4, value_edit.confine_height(3, Cling::Top)),
		]);
		let radiate_state = if state.facet.is_valid() && state.point.is_valid() && state.value.is_valid() {
			ButtonState::enabled(SenderLink::ignore())
		} else {
			ButtonState::disabled()
		};
		let content = edits
			.pack_top(3, yard::title("Editor", StrokeColor::BodyOnBackground, Cling::LeftTop))
			.pack_bottom(1, yard::button("Publish", radiate_state).confine_width(20, Cling::Center))
			.pad(4).pad_cols(4);
		Some(content.pack_left(15, sidebar))
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum YardId {
	FacetEdit,
	PointEdit,
	ValueEdit,
	EditList,
}

impl Into<i32> for YardId {
	fn into(self) -> i32 {
		let mut hasher = DefaultHasher::new();
		self.hash(&mut hasher);
		hasher.finish() as i32
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	yui::main(MainSpark {})?;
	Ok(())
}
