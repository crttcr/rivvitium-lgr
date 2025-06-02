use riv::model::ir::atom::Atom;
use riv::model::ir::external_metadata::FileMetadata;
use riv::model::ir::external_metadata::SourceVariant;
use riv::model::ir::nv_strings::NVStrings;
use riv::utils::test_file::TestFile;

pub struct TestAtoms {}

impl TestAtoms {
	
	pub fn create_start_file_atom() -> Atom {
		// 1) spin up a temp file with known content
		let x = TestFile::with_content("hello, world!").expect("couldn't make temp file");
		let f = x.path_string();
		let v = FileMetadata::for_file(f, None).unwrap();
		let v = SourceVariant::File(v);
		Atom::StartTask(v)
	}
	
	pub fn start_end_vec() -> Vec<Atom> { 
		let a = Self::create_start_file_atom();
		let b = Atom::EndTask;
		vec![a,b]
	}
		
	pub fn nv_pairs() -> Vec<Atom> { 
		let nv_a = NVStrings::new(vec![
			("City".to_string(),   "Tokyo".to_string()),
			("Temperature".to_string(),   "35.6897".to_string()),
		]);	

		let nv_b = NVStrings::new(vec![
			("City".to_string(),   "Jakarta".to_string()),
			("Temperature".to_string(),   "-6.175".to_string()),
		]);	

		let nv_c = NVStrings::new(vec![
			("City".to_string(),   "Delhi".to_string()),
			("Temperature".to_string(),   "28.61".to_string()),
		]);	

		let a = Self::create_start_file_atom();
		let b = Atom::StringNVAtom(nv_a);
		let c = Atom::StringNVAtom(nv_b);
		let d = Atom::StringNVAtom(nv_c);
		let e = Atom::EndTask;
		vec![a,b,c,d,e]
	}
}	

