use std::collections::HashMap;
use std::hash::Hash;
use ulid::Ulid;
use crate::model::ir::external_metadata::ExternalMetadataVariant::File;
use crate::utils::digest::sha256_digest_file;

const META_CORRELATION_ID: &str = "correlation_id";
const META_FILE_NAME:      &str = "file_name";
const META_SHA_256:        &str = "sha_256";
const META_BYTE_COUNT:     &str = "byte_count";

type ExternalMetadataMap = HashMap<String, String>;

// Common behavior shared by external metadata
//
trait ExternalMetadata {
	fn content(&self) -> ExternalMetadataMap;
}

// Individual "subtypes" of metadata
//

#[derive(Debug)]
pub struct FileMetadata {
	pub ulid:            Ulid,
	pub filename:        String,
	pub sha256:          String,
	pub correlation_id:  String,
	pub byte_count:      u64,
}

impl FileMetadata {
	pub fn for_file(name: String, cid: Option<String>) -> FileMetadata {
		let ulid           = Ulid::new();
		let filename       = name;
		let sha256         = sha256_digest_file(filename.as_str()).expect("Couldn't hash file");
		let correlation_id = cid.unwrap_or(ulid.to_string());
		let byte_count     = 0;
		FileMetadata {ulid, filename, sha256, correlation_id, byte_count }
	}
}

// Implement the common behavior
//
impl ExternalMetadata for FileMetadata {
	fn content(&self) -> ExternalMetadataMap {
		let mut map = HashMap::new();
		map.insert(META_BYTE_COUNT    .to_string(), self.byte_count    .to_string());
		map.insert(META_CORRELATION_ID.to_string(), self.correlation_id.to_string());
		map.insert(META_FILE_NAME     .to_string(), self.filename      .to_string());
		map.insert(META_SHA_256       .to_string(), self.sha256        .to_string());
		map
	}
}

// Enum to represent each kind of metadata
//
#[derive(Debug)]
pub enum ExternalMetadataVariant {
	File(FileMetadata),
//	Message(ImageContent),
//	API(ApiMetadata),
//	HTTP(HttpMetadata),
}

// Implement the common behavior on the enum variants using
// match + delegation
//
impl ExternalMetadata for ExternalMetadataVariant {
	fn content(&self) -> ExternalMetadataMap {
		match self {
			File(f) => f.content(),
		}
	}
}
