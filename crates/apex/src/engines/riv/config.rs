use riv::component::relay::{RelayConfig};
use riv::component::sink::{SinkConfig};
use riv::component::source::SourceConfig;
use std::error::Error;
use riv::component::source::csv_string_source::CsvStringSource;
use crate::engines::riv::engine::Engine;

#[derive(Default)]
pub struct Config {
    source:   Option<Box<dyn SourceConfig>>,
    relays:   Vec<Box<dyn RelayConfig>>,
    sink:     Option<Box<dyn SinkConfig>>,
}

impl Config {
    pub fn new() -> Self { Self::default() }

    // Predicates
    pub fn can_parse(&self)    -> bool {self.source.is_some()}
    pub fn can_publish(& self) -> bool {self.source.is_some() && self.sink.is_some()}

    pub fn source(&mut self, src: Box<dyn SourceConfig>) {
        self.source = Some(src);
        println!("PB: I now have a source");
    }

    pub fn source_reset(&mut self) {
        self.source = None;
    }

    pub fn add_relay(mut self, relay: Box<dyn RelayConfig>) {
        self.relays.push(relay);
        println!("PB: I have a new relay");
    }

    pub fn relay_clear(&mut self) {
        self.relays.clear();
        println!("PB: I have a no relays, fo' sho'");
    }

    pub fn sink(mut self, sink: Box<dyn SinkConfig>) {
        self.sink = Some(sink);
        println!("PB: I now have a sink");
    }

    /// Convert the builder into a usable `ProcessingPipeline`.
    ///
    /// Only `source` is mandatory; relays may be empty and
    /// sink is optional.
    pub fn build(&self) -> Result<Engine, Box<dyn Error>> {
        let source = self.source.as_ref().ok_or("PipelineBuilder must have a source")?;
        let path   = source.path_buf().ok_or("PipelineBuilder must have a source path")?;
			let path = path.clone().into_os_string().into_string().expect("Path to string failed");
        let source = CsvStringSource::new(path);
        let source = Box::new(source);
        let relays = vec![];
        let sink = None;
        Ok(Engine {
            source,
            relays,
            sink,
        })
    }
}

/* ───────── example usage ─────────────────────────────────────────

let engine = PipelineBuilder::new()
    .source(FileSource::new("data.csv"))
    .add_relay(DedupeRelay::default())
    .sink(ConsoleSink)
    .build()?;
assert!(engine.has_sink());
*/
