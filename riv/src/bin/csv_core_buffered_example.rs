use csv_core::{ReaderBuilder, ReadRecordResult};
use std::fs::File;
use std::{io, io::Read};

const DATA_PATH:  &str  = "/Users/reid/Coding/Rust/LGR/rivvitium-lgr/auxbox/data/";
const DATA_FILE:  &str  = "weather_stations.csv";
const CHUNK_SIZE: usize = 4 * 1024;     // read file in 4 KiB chunks
const OUT_SIZE:   usize = 8 * 1024;     // unescaped record buffer
const ENDS_SIZE:  usize =      128;     // max fields per record

fn main() -> io::Result<()> {
	let file     = DATA_PATH.to_owned() + DATA_FILE;                        // 1) Open the file
	let mut file = File::open(file)?;

	let mut parser   = ReaderBuilder::new().delimiter(b';').build();        // 2) Prepare csv_core::Reader and buffers
	let mut file_buf = [0u8; CHUNK_SIZE];
	let mut out_buf  = vec![0u8; OUT_SIZE];
	let mut ends_buf = vec![0usize; ENDS_SIZE];
	let mut start    = 0;                            // `start`..`end` marks the valid data in `file_buf`
	let mut end      = 0;

	loop {
		if start == end {                                                    // 3) Do we need a refill?
			let n = file.read(&mut file_buf)?;
			if n == 0 {break}                           // EOF and no leftover → we’re done
			start = 0;
			end   = n;
		}

		let input = &file_buf[start..end];                                   // 4) Hand the available slice to csv_core
		let (status, bytes_read, bytes_written, field_count) = parser.read_record(input, &mut out_buf, &mut ends_buf);
		start += bytes_read;                                                 // 5) Advance window by `bytes_read` bytes

		match status {
			ReadRecordResult::Record => {
				let record_str = String::from_utf8_lossy(&out_buf[..bytes_written]);      // Got a full record in out_buf[0..bytes_written]
				let line       = &ends_buf[..field_count]
					.iter()
					.map(|v| format!("{:>width$}", v, width=3))
					.collect::<Vec<_>>()
					.join(",");
				println!("{}: {}", line, record_str.trim_end());
			}
			ReadRecordResult::InputEmpty       => { continue; }              // Need more input: loop back to refill if possible
			ReadRecordResult::End              => { break; }                          // No more records (trapped EOF in the middle of parser)
			ReadRecordResult::OutputFull       => { panic!("Record too large for OUT_SIZE buffer!"); }
			ReadRecordResult::OutputEndsFull   => { panic!("Too many fields for ENDS_SIZE!"); }
		}
	}
	Ok(())
}
