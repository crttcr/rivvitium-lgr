
use riv::component::source::SourceConfig;

/* ───────── example usage ─────────────────────────────────────────

let pipeline = ProcessingPipelineBuilder::new()
    .source(FileSource::new("data.csv"))
    .add_relay(DedupeRelay::default())
    .sink(ConsoleSink)
    .build()?;
assert!(pipeline.has_sink());
*/
/*
use riv::component::source::Source;
use riv::component::source::vector_source::VectorSource;
use crate::engines::riv::processing_pipeline::ProcessingPipelineBuilder;

#[test]
fn accepts_multiple_relays() {
	let pipeline = ProcessingPipelineBuilder::new()
		.source(make_source())
		.add_relay(DummyRelay)
		.add_relay(DummyRelay)
		.add_relay(DummyRelay)
		.build()
		.unwrap();

	assert_eq!(pipeline.relays.len(), 3);
}

fn make_source() -> Box<dyn Source> {
	let vec = vec![];
	let src = VectorSource::new(vec);
	Box::new(src)
}

*/
use riv::component::source::path_buf_config::PathBufConfig;
use zero::util::file_utils::{bogus_path, make_temp_file_named};
use crate::engines::riv::config::Config;

#[test]
fn with_source_accepts_readable_file() {
	let mut cfg  = Config::new();
	let pbuf     = make_temp_file_named("george.csv");
	let src_cfg  = PathBufConfig::new(pbuf);
	let src_cfg  = Box::new(src_cfg);
	cfg.source(src_cfg);
	let parse_ok = cfg.can_parse();
	println!("{:#?}", parse_ok);
	assert!(parse_ok);
 }

#[test]
fn with_source_rejects_unreadable_file() {
	let mut cfg  = Config::new();
	let bogus    = bogus_path();
	let src_cfg  = PathBufConfig::new(bogus);
	let src_cfg  = Box::new(src_cfg);
	cfg.source(src_cfg);
	let parse_ok = cfg.can_parse();
	println!("{:#?}", parse_ok);
	assert!(!parse_ok);
}