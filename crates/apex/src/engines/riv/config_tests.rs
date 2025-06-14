
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
