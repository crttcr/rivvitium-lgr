
pub mod button_bar;
pub mod footer;
pub mod header;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ApplicationStatus {
	#[default]
	NotConfigured,
	Idle,
	Running,
}
