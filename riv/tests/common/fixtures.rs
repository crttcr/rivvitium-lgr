use riv::model::ir::atom::Atom;
use riv::model::ir::external_metadata::FileMetadata;
use riv::model::ir::external_metadata::ExternalMetadataVariant;

pub struct TestAtoms {}

impl TestAtoms {
	
	pub fn create_start_file_atom() -> Atom {
		let f = "/tmp/test_initialize_and_iterate".to_string();
		let v = FileMetadata::for_file(f, None);
		let v = ExternalMetadataVariant::File(v);
		Atom::Start(v)
	}
	
	pub fn start_end_vec() -> Vec<Atom> { 
		let a = Self::create_start_file_atom();
		let b = Atom::Finish;
		vec![a,b]}
}
