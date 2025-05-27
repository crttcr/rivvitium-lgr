use std::fmt;
use crate::model::coordinate::text_location::TextLocation;

/// Coordinates describe locations in
///
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Coordinate {
	Undefined,
	Position(TextLocation),
	Extent{start: TextLocation, end: TextLocation},
}

impl Coordinate {
	// Construction
	pub fn at_location(v: TextLocation) -> Self { Coordinate::Position(v) }
	pub fn for_range(start: TextLocation, end: TextLocation) -> Self {
		Coordinate::Extent {start, end}
	}

	// Predicates
	pub fn is_none(&self) -> bool { *self == Coordinate::Undefined }

	// Convenience methods
	pub fn byte_count(&self) -> u64 {
		match self {
			Coordinate::Undefined          => 0,
			Coordinate::Position(_)        => 1,
			Coordinate::Extent{start, end} => end.byte - start.byte + 1,

		}
	}
}

impl fmt::Display for Coordinate {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Coordinate::Undefined          => write!(f, "Undefined coordinate"),
			Coordinate::Position(v)        => write!(f, "Position({:?})", v),
			Coordinate::Extent{start, end} => write!(f, "Extent({:?} -> {:?})", start, end),
		}
	}
}

impl fmt::Debug for Coordinate {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Coordinate::Undefined          => write!(f, "Coordinate::None"),
			Coordinate::Position(v)        => write!(f, "Coordinate::Position({:?})", v),
			Coordinate::Extent{start, end} => write!(f, "Coordinate::Extent({:?} -> {:?})", start, end),
		}
	}
}	

