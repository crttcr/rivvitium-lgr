
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum AtomType
{
	Control,     // Errors, Warnings, Start, Completion
	Data,        // Positional data, Name value pairs,
	Metadata,    // Filename, Headers, Parent-Child relationships
}


#[test]
pub fn display_mote_type() {
	let a = AtomType::Control;
	let b = AtomType::Metadata;
	let c = AtomType::Data;
	println!("{:?}", a);
	println!("{:?}", b);
	println!("{:?}", c);
}
