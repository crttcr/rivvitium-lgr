mod common;

use riv::component::relay::console_relay::ConsoleRelay;
use riv::component::relay::Relay;
use riv::component::source::csv_string_source::CsvStringSource;
use riv::component::source::Source;
use riv::Error;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::fmt::format::FmtSpan;
use riv::component::relay::empty_relay_config::EmptyRelayConfig;
use common::fixtures::TestComponents;
use common::fixtures::TestFiles;

#[test]
pub fn test_csv_pipeline() -> Result<(), Error> {
	// Startup
	let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
	fmt::Subscriber::builder()
		.with_env_filter(filter)
		.with_span_events(FmtSpan::ENTER) //  | FmtSpan::EXIT)
		.init();

	tracing::info!("Creating pipeline components");
	let file_name  = TestFiles::weather_file_10_name_as_string();
	let mut src    = CsvStringSource::new(file_name);
	let mut relay  = ConsoleRelay::new();
	let (target_cfg, mut dst) = TestComponents::csv_config_and_sink(401, "/tmp/test_csv_pipeline.csv");

	tracing::info!("Initializing pipeline components");
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
	dst.close();
	assert!(source_ok);
	assert!(relay_ok);
	Ok(())
}
