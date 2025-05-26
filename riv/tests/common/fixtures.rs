use riv::model::ir::atom::Atom;
use riv::model::ir::external_metadata::FileMetadata;
use riv::model::ir::external_metadata::TaskVariant;
use riv::utils::test_file::TestFile;

pub struct TestAtoms {}

impl TestAtoms {
	
	pub fn create_start_file_atom() -> Atom {
		// 1) spin up a temp file with known content
		let x = TestFile::with_content("hello, world!").expect("couldn't make temp file");
		let f = x.path_string();
		let v = FileMetadata::for_file(f, None).unwrap();
		let v = TaskVariant::File(v);
		Atom::StartTask(v)
	}
	
	pub fn start_end_vec() -> Vec<Atom> { 
		let a = Self::create_start_file_atom();
		let b = Atom::Finish;
		vec![a,b]}
}

