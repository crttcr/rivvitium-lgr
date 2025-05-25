use riv::model::ir::atom::Atom;


pub struct TestAtoms {}

impl TestAtoms {
	pub fn start_end_vec() -> Vec<Atom> { vec![Atom::Start, Atom::Finish] }
}
