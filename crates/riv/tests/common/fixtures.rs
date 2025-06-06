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

	pub fn create_end_file_atom() -> Atom {
		Atom::EndTask
	}

	pub fn start_end_vec() -> Vec<Atom> {
		let a = Self::create_start_file_atom();
		let b = Self::create_end_file_atom();
		vec![a,b]
	}

	pub fn wrap_with_start_end(atoms: Vec<Atom>) -> Vec<Atom> {
		let a = Self::create_start_file_atom();
		let b = Self::create_end_file_atom();
		let mut rv = atoms; // Take ownership of the input Vec
		rv.insert(0, a);    // Prepend 'a' at the beginning
		rv.push(b);         // Append 'b' at the end
		rv
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

		let a = Atom::StringNVAtom(nv_a);
		let b = Atom::StringNVAtom(nv_b);
		let c = Atom::StringNVAtom(nv_c);
		let unwrapped = vec![a, b, c];
		Self::wrap_with_start_end(unwrapped)
	}
}
