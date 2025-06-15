use riv::component::sink::sink_settings::SinkSettings;
use riv::component::sink::SinkKind;
use riv::model::ir::atom::Atom;
use riv::model::ir::external_metadata::FileMetadata;
use riv::model::ir::external_metadata::SourceVariant;
use riv::model::ir::nv_strings::NVStrings;
use riv::utils::test_file::TestFile;
use zero::component::identity::id_generator::global_id_gen;

pub struct TestAtoms      {}
pub struct TestComponents {}
pub struct TestFiles      {}

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

impl TestComponents {
	pub fn csv_config_and_sink(component_id: u32, out_file_name: &str) -> (SinkSettings, Box<dyn riv::component::sink::Sink>) {
		let cfg = SinkSettings::csv(out_file_name, ';');
		let (tx, _)    = std::sync::mpsc::channel();
		let dst = cfg.build_sink(component_id, tx).unwrap();
		(cfg, dst)
	}

	pub fn capture_config_and_sink() -> (SinkSettings, Box<dyn riv::component::sink::Sink>) {
		let cid = global_id_gen().next_id();
		let cfg = SinkSettings::capture();
		let (tx, _)    = std::sync::mpsc::channel();
		let dst = cfg.build_sink(cid, tx).unwrap();
		(cfg, dst)
	}
}

impl TestFiles {
	pub fn weather_file_10_name_as_string() -> String {
		"../../auxbox/data/weather_stations.10.csv".to_owned()
	}
}

#[test]
fn test_file_exists() {
	let f = TestFiles::weather_file_10_name_as_string();
		assert!(std::path::Path::new(&f).exists());
}

#[test]
fn test_build_the_stupid_component() {
	let id     = global_id_gen().next_id();
	let (a, b) = TestComponents::csv_config_and_sink(id, "test.csv");
	assert!(a.kind() == SinkKind::Csv);
	assert!(b.kind() == SinkKind::Csv);
}

#[test]
fn test_build_another_stupid_component() {
	let (a, b) = TestComponents::capture_config_and_sink();
	assert!(a.kind() == SinkKind::Capture);
	assert!(b.kind() == SinkKind::Capture);
}

#[test]
fn wrap_with_start_end_test() {
	let atoms   = Vec::new();
	let wrapped = TestAtoms::wrap_with_start_end(atoms);
	println!("{:?}", wrapped);
	let head = wrapped.get(0).unwrap();
	let tail = wrapped.get(wrapped.len()-1).unwrap();
	println!("{:?}", head);
	println!("{:?}", tail);
}
