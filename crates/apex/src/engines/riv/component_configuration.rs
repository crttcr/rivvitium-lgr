use riv::component::relay::{RelayConfig};
use riv::component::source::{SourceConfig, SourceType};
use std::error::Error;
use std::sync::mpsc::Sender;
use riv::component::sink::sink_settings::SinkSettings;
use riv::component::source::csv_string_source::CsvStringSource;
use zero::component::telemetry::component_metrics::ComponentMetrics;
use crate::engines::riv::engine::Engine;

#[derive(Default)]
pub struct ComponentConfiguration {
    source:   Option<Box<dyn SourceConfig>>,
    relays:   Vec<Box<dyn RelayConfig>>,
    sink:     SinkSettings,
}

impl ComponentConfiguration {
    pub fn new() -> Self { Self::default() }

    // Predicates
    pub fn can_parse(&self)    -> bool {self.source.is_some()}
    pub fn can_publish(& self) -> bool {self.source.is_some() && self.sink.can_publish()}

    pub fn set_source_configuration(&mut self, src: Box<dyn SourceConfig>) {
        self.source = Some(src);
    }
    
    pub fn source_configuration_type(&self) -> Option<SourceType > {
        self.source
        	.as_ref()
        	.map(|boxed| boxed.source_type())
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

    pub fn get_sink_configuration(&self) -> SinkSettings {
        self.sink.clone()
    }
    
    pub fn set_sink_configuration(&mut self, sink: &SinkSettings) {
        self.sink = sink.clone();
        println!("PB: I now have a sink");
    }

    /// Convert the builder into a usable `ProcessingPipeline`.
    ///
    pub fn build(&self, _: Sender<ComponentMetrics>) -> Result<Engine, Box<dyn Error>> {
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
