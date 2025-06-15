use std::fs::File;
use riv::component::relay::console_relay::ConsoleRelay;
use riv::component::relay::Relay;
use riv::component::source::Source;
use riv::{data_file_path_as_str, Error};
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::fmt::format::FmtSpan;
use riv::component::relay::empty_relay_config::EmptyRelayConfig;
use riv::component::sink::sink_settings::SinkSettings;
use riv::component::source::csv_byte_source::CsvByteSource;


#[test]
pub fn run_csv_byte_pipeline() -> Result<(), Error> {
	// Startup
	let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
	fmt::Subscriber::builder()
		.with_env_filter(filter)
		.with_span_events(FmtSpan::ENTER) //  | FmtSpan::EXIT)
		.init();

	tracing::info!("Creating pipeline components");
	let file_path = data_file_path_as_str("weather_stations.10.csv");
	let file      =  File::open(file_path).expect("File open failed");
	let mut src   = CsvByteSource::new(file);
	let mut relay = ConsoleRelay::new();
	let target_cfg = SinkSettings::csv("test.output.csv", ';');
	let (tx, _)    = std::sync::mpsc::channel();
	let mut dst    = target_cfg.build_sink(29, tx)?;

	tracing::info!("Initializing pipeline components");
	let relay_cfg       = EmptyRelayConfig::default();
	let relay_msg       = relay.initialize(&relay_cfg)?;
	let target_msg      = dst.initialize(&target_cfg)?;

	assert_eq!(relay_msg,  ());
	assert_eq!(target_msg, ());

	tracing::info!("Pulling data through the pipeline");
	for atom in &mut src {
		let _ = match relay.accept(atom) {
			Some(revised) => dst.accept(revised),
			None          => Ok(()),
		};
	}

	tracing::info!("Finishing components");
	let source_ok = src.close()?;
	let relay_ok  = relay.finish();
	dst.close();
	assert!(source_ok);
	assert!(relay_ok);
	Ok(())
}

