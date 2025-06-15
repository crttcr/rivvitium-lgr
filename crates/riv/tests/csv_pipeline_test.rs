use riv::component::relay::console_relay::ConsoleRelay;
use riv::component::relay::Relay;
use riv::component::sink::Sink;
use riv::component::source::csv_string_source::CsvStringSource;
use riv::component::source::Source;
use riv::Error;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::fmt::format::FmtSpan;
use riv::component::relay::empty_relay_config::EmptyRelayConfig;
use riv::component::sink::csv_sink::CsvSink;

#[test]
pub fn test_csv_pipeline() -> Result<(), Error> {
	// Startup
	let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
	fmt::Subscriber::builder()
		.with_env_filter(filter)
		.with_span_events(FmtSpan::ENTER) //  | FmtSpan::EXIT)
		.init();

	tracing::info!("Creating pipeline components");
	let file_name = "../auxbox/data/weather_stations.10.csv".to_owned();
	let mut src   = CsvStringSource::new(file_name);
	let mut relay = ConsoleRelay::new();
	let mut dst   = CsvSink::new("csv_string_output.csv".to_string());

	tracing::info!("Initializing pipeline components");
	let cfg             = "TODO: Use configuration".to_owned();
	let target_cfg      = EmptySinkConfig::default();
	let target_msg      = dst.initialize(&target_cfg)?;
	let relay_cfg       = EmptyRelayConfig::default();
	let relay_msg       = relay.initialize(&relay_cfg)?;

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
	let count     = dst.close();
	assert!(source_ok);
	assert!(relay_ok);
	Ok(())
}
