use crate::error::{Error, IoErrorWrapper};
use crate::model::ir::external_metadata::TaskVariant::Bytes;
use crate::model::ir::external_metadata::TaskVariant::File;
use crate::utils::digest::*;
use std::collections::HashMap;
use std::ffi::OsString;
use std::hash::Hash;
use ulid::Ulid;

const META_CORRELATION_ID: &str = "correlation_id";
const META_FILE_NAME:      &str = "file_name";
const META_SHA_256:        &str = "sha_256";
const META_BYTE_COUNT:     &str = "byte_count";

type ExternalMetadataMap = HashMap<String, String>;

// Common behavior shared by external metadata
//
trait TaskMetadata {
	fn as_map(&self)         -> ExternalMetadataMap;
	fn correlation_id(&self) -> &str;
	fn sha_256(&self)        -> &str;
	fn ulid(&self)           -> &Ulid;
}

// Individual "subtypes" of metadata
//
#[derive(Debug)]
pub struct BytesMetadata {
	pub ulid:            Ulid,
	pub sha256:          String,
	pub correlation_id:  String,
	pub byte_count:      u64,
}

impl BytesMetadata {
	pub fn for_bytes(bytes: &[u8], cid: Option<String>) -> BytesMetadata {
		let ulid           = Ulid::new();
		let sha256         = sha256_digest_bytes(bytes);
		let correlation_id = cid.unwrap_or(ulid.to_string());
		let byte_count     = bytes.len() as u64;
		BytesMetadata{ulid, sha256, correlation_id, byte_count}
	}
}

// Implement the common behavior
//
impl TaskMetadata for BytesMetadata {
	fn sha_256(&self)        -> &str  {  self.sha256.as_str()         }
	fn correlation_id(&self) -> &str  {  self.correlation_id.as_str() }
	fn ulid(&self)           -> &Ulid { &self.ulid                    }
	
	fn as_map(&self) -> ExternalMetadataMap {
		let mut map = HashMap::new();
		map.insert(META_BYTE_COUNT    .to_string(), self.byte_count    .to_string());
		map.insert(META_CORRELATION_ID.to_string(), self.correlation_id.to_string());
		map.insert(META_SHA_256       .to_string(), self.sha256        .to_string());
		map
	}
}

#[derive(Debug)]
pub struct FileMetadata {
	pub ulid:            Ulid,
	pub filename:        String,
	pub sha256:          String,
	pub correlation_id:  String,
	pub byte_count:      u64,
}

impl FileMetadata {
	pub fn for_file(name: String, cid: Option<String>) -> Result<FileMetadata, Error> {
		let ulid           = Ulid::new();
		let filename       = name;
		let sha256         = sha256_digest_file(filename.as_str()).map_err(IoErrorWrapper::from)?;
		let correlation_id = cid.unwrap_or(ulid.to_string());
		let byte_count     = 0;
		let rv             = FileMetadata {ulid, filename, sha256, correlation_id, byte_count};
		Ok(rv)
	}
}

impl TaskMetadata for FileMetadata {
	fn sha_256(&self)        -> &str  {  self.sha256.as_str()         }
	fn correlation_id(&self) -> &str  {  self.correlation_id.as_str() }
	fn ulid(&self)           -> &Ulid { &self.ulid                    }
	fn as_map(&self) -> ExternalMetadataMap {
		let mut map = HashMap::new();
		map.insert(META_BYTE_COUNT    .to_string(), self.byte_count    .to_string());
		map.insert(META_CORRELATION_ID.to_string(), self.correlation_id.to_string());
		map.insert(META_FILE_NAME     .to_string(), self.filename      .clone());
		map.insert(META_SHA_256       .to_string(), self.sha256        .to_string());
		map
	}
}

// Enum to represent each kind of metadata
//
#[derive(Debug)]
pub enum TaskVariant {
	Bytes(BytesMetadata),
	File(FileMetadata),
//	Message(ImageContent),
//	API(ApiMetadata),
//	HTTP(HttpMetadata),
}

// Implement the common behavior on the enum variants using
// match + delegation
//
impl TaskMetadata for TaskVariant {
	fn sha_256(&self) -> &str { 
		match self {
			Bytes(v) => v.sha256.as_str(),
			File(v)  => v.sha256.as_str(),
		}
	}
		
	fn correlation_id(&self) -> &str { 
		match self {
			Bytes(v) => v.correlation_id.as_str(),
			File(v)  => v.correlation_id.as_str(),
		}
	}

	fn as_map(&self) -> ExternalMetadataMap {
		match self {
			Bytes(v) => v.as_map(),
			File(v)  => v.as_map(),
		}
	}
		
	fn ulid(&self) -> &Ulid {
		match self {
			Bytes(v) => v.ulid(),
			File(v)  => v.ulid(),
		}
	}
}
