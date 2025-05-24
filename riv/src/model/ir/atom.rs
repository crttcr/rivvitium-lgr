use crate::model::ir::atom_type::AtomType;


#[derive(Debug)]
pub enum Atom
{
	// Control
	Start,
	
	// Data
	Values(u8),
	
	// Metadata
//	Metadata(Metadata),
	
}

impl Atom {
	fn atom_type(&self) -> AtomType {
		match self 
		{
			Atom::Start     => AtomType::Control,
			Atom::Values(_) => AtomType::Data,
		}
	}
}


/*
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