use crate::component::source::vector_source::VectorSource;
use crate::model::ir::atom::Atom;
use crate::model::ir::external_metadata::{ExternalMetadataVariant, FileMetadata};

#[test]
fn file_metadata_has_a_valid_ulid() {
	let f = "/tmp/test_initialize_and_iterate".to_string();
	let a = FileMetadata::for_file(f, None);
	let s = a.ulid.to_string();
	println!("{}", s);
	println!("{a:?}");
	println!("{:?}", a.ulid.to_string());
	}
