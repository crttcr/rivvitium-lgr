use crate::Error;
use crate::model::ir::atom_type::AtomType;
use crate::model::ir::byte_row::ByteRow;
use crate::model::ir::external_metadata::TaskVariant;
use crate::model::ir::string_row::StringRow;

#[derive(Debug)]
pub enum Atom
{
	// Control
	StartDocument(TaskVariant),
	EndDocument,
	ErrorAtom(Error),
	
	// Data
	StringRowAtom(StringRow),  // For sources that only enable strings
	ByteRowAtom(ByteRow),      // For sources that supply raw bytes
	NamedValues(u8),           // TODO: Figure out how I want to model this ...
	
	// Metadata
	HeaderRow(StringRow),
	CommentRow(String),
	BlankLine,
	InternalMetadata           // TODO: Model this
}

impl Atom {
	fn atom_type(&self) -> AtomType {
		match self 
		{
			Atom::HeaderRow(_)      => AtomType::Metadata,
			Atom::NamedValues(_)    => AtomType::Data,
			Atom::StringRowAtom(_)  => AtomType::Data,
			Atom::ByteRowAtom(_)    => AtomType::Data,
			_                       => AtomType::Control,
		}
	}
}


// TODO: This should be modeled as its own concept where it can be tested
//
/*
// TODO: InternalMetadata
// Header
// Section
//   Binding(n = v)
//   Binding
// Trailer
//
// Example below:
//
View trades
TRADE PARAMETERS : 
Trade Start Date = 31-JAN-2025
Trade End Date = 17-MAR-2025
 PortGroup = XXX
TRANSACTION PARAMETERS: 
 Include Transaction Type = BB, CADJ, ROLL, TRD, ISSU, ALLC
 SECURITIES PARAMETERS : 
 RatingOp = >
 Exclude Security Group/Type = [FUND/STIF]
 Reason = DEFAULT,WSTO
 COUNTERPARTY PARAMETERS : 
Counterparty Type = [BROKER]
133 matches found
*/