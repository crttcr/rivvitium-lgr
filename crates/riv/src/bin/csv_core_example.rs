use std::fs::File;
use std::io::Read;
use csv_core::{ReaderBuilder, ReadRecordResult};

const DATA_PATH: &str = "/Users/reid/Coding/Rust/LGR/rivvitium-lgr/auxbox/data/";

fn main() -> std::io::Result<()> {

	// Prepare data, csv_core reader, and output buffers
	//
	let data         = read_file("weather_stations.10.csv")?;
	let mut reader   = ReaderBuilder::new().delimiter(b';').build();
	let mut input    = data.as_slice();
	let mut out_buf  = vec![0u8; 8 * 1024];
	let mut ends_buf = vec![0usize; 128];

	// Loop until all input consumed
	//
	loop {
		let (result, bytes_read, bytes_written, field_count) = reader.read_record(input, &mut out_buf, &mut ends_buf);
		input = &input[bytes_read..];                           // Prep for next read
		match result {
			ReadRecordResult::End            => break,           // All has been read
			ReadRecordResult::InputEmpty     => break,
			ReadRecordResult::OutputFull     => { panic!("output buffer too small for record"); }
			ReadRecordResult::OutputEndsFull => { panic!("ends buffer too small for record"); }
			ReadRecordResult::Record         => {               // The end of the record was found
				let record_str = String::from_utf8_lossy(&out_buf[..bytes_written]);      // out_buf[..bytes_written] is the unescaped record
				let line       = &ends_buf[..field_count]
					.iter()
					.map(|v| format!("{:>width$}", v, width=3))
					.collect::<Vec<_>>()
					.join(",");
				println!("{}: {}", line, record_str.trim_end());
			}
		}
	}

	Ok(())
}

// 1. Read entire file into memory as a byte vector
//
fn read_file(file: &str) -> std::io::Result<Vec<u8>> {
	let path = DATA_PATH.to_owned() + file;
	let mut file = File::open(path)?;
	let mut data = Vec::new();
	file.read_to_end(&mut data)?;
	Ok(data)
}