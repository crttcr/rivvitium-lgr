
pub mod action_panel;
pub mod button_bar;
pub mod footer;
pub mod header;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ActiveAction {
	#[default]
	Home,
	Ready,
	Run,
	Result,
}
