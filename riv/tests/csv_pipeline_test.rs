use riv::component::relay::console_relay::ConsoleRelay;
use riv::component::relay::Relay;
use riv::component::sink::capture_sink::CaptureSink;
use riv::component::sink::Sink;
use riv::component::source::csv_source::CsvSource;
use riv::component::source::Source;
use riv::Error;

#[test]
pub fn test_csv_pipeline() -> Result<(), Error> {

	// Create pipeline components
	let file_name = "../aux/data/weather_stations.10.csv".to_owned();
	let mut src   = CsvSource::new(file_name);
	let mut relay = ConsoleRelay::new();
	let mut dst   = CaptureSink::new();

	// Initialize pipeline components
	let cfg             = "cfg".to_owned();
	let source_msg      = src.initialize(&cfg)?;
	let relay_msg       = relay.initialize(&cfg)?;
	let target_msg      = dst.initialize(&cfg)?;

	assert_eq!(source_msg, ());
	assert_eq!(relay_msg,  ());
	assert_eq!(target_msg, ());

	for atom in &mut src {
		let _ = match relay.accept(atom) {
			Some(revised) => dst.accept(revised),
			None          => Ok(()),
		};
	}

	let source_ok = src.finish()?;
	let relay_ok  = relay.finish();
	let count    = dst.finish()?;
	assert!(source_ok);
	assert!(relay_ok);
	println!("{:?}", count);
	Ok(())
}
