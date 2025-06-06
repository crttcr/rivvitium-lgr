use std::fmt;

/// TextLocation pinpoints a position in some source text.
/// Intended uses are to
/// * highlight a text region, and 
/// * enrich error reports. 
///
/// Line numbers start at 1.
/// Column numbers start at 1.
/// Byte numbers start from 0.
///
#[derive(Eq, PartialEq, Clone, Copy)]
pub struct TextLocation
{
	pub line:   u64,
	pub column: u32,
	pub byte:   u64,
}

impl fmt::Display for TextLocation
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}:{} byte:{}", self.line, self.column, self.byte)
	}
}

impl fmt::Debug for TextLocation
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "line:{}, column:{}, byte:{}", self.line, self.column, self.byte)
	}
}

impl TextLocation
{
	pub fn new(byte: u64, line: u64, column: u32) -> Self {
		Self {byte, line, column}
	}
}
