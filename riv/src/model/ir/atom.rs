use crate::Error;
use crate::model::ir::atom_type::AtomType;
use crate::model::ir::data_record::DataRecord;
use crate::model::ir::external_metadata::TaskVariant;


#[derive(Debug)]
pub enum Atom<'a>
{
	// Control
	StartTask(TaskVariant),
	FinishTask,
	ErrorAtom(Error),
	
	// Data
	ValueSequence(DataRecord<'a>),
	NamedValues(u8),        // TODO: Figure out how I want to model this ...
	
	// Metadata
	HeaderRow(DataRecord<'a>),
	CommentRow(String),
	BlankLine,
	InternalMetadata         // TODO: Model this
}

impl<'a> Atom<'a> {
	fn atom_type(&self) -> AtomType {
		match self 
		{
			Atom::HeaderRow(_)      => AtomType::Metadata,
			Atom::NamedValues(_)    => AtomType::Data,
			Atom::ValueSequence(_)  => AtomType::Data,
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