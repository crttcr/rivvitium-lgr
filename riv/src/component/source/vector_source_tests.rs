use crate::component::source::vector_source::VectorSource;
use crate::component::source::Source;
use crate::error::Error;
use crate::model::ir::atom::Atom;
use crate::model::ir::external_metadata::{BytesMetadata, SourceVariant};

#[test]
fn test_initialize_and_iterate() -> Result<(), Error> {
	let bytes     = "test_initialize_and_iterate".as_bytes();
	let meta      = BytesMetadata::for_bytes(bytes, None);
	let variant   = SourceVariant::Bytes(meta);
	let atom      = Atom::StartTask(variant);
	let atoms     = vec![atom, Atom::EndTask, ];
	let mut src   = VectorSource::new(atoms);
	let collected = src.by_ref().collect::<Vec<_>>();
	println!("{:?}", collected);	
	// After completion, next yields None
	assert!(src.next().is_none());
	let rv = src.finish()?;
	assert!(rv);
	Ok(())
}
