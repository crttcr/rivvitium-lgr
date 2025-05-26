use crate::Error;
use crate::model::ir::atom_type::AtomType;
use crate::model::ir::external_metadata::TaskVariant;


#[derive(Debug)]
pub enum Atom
{
	// Control
	StartTask(TaskVariant),
	FinishTask,
	ErrorAtom(Error),
	
	// Data
	RawValues(u8),
	NamedValues(u8),
	
	// Metadata
	HeaderRow(Vec<String>),
//	Metadata(Metadata),
	
}

impl Atom {
	fn atom_type(&self) -> AtomType {
		match self 
		{
			Atom::StartTask(_)    => AtomType::Control,
			Atom::FinishTask       => AtomType::Control,
			Atom::ErrorAtom(_)     => AtomType::Control,
			Atom::NamedValues(_)   => AtomType::Data,
			Atom::RawValues(_)     => AtomType::Data,
			Atom::HeaderRow(_)     => AtomType::Metadata,
		}
	}
}


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