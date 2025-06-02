use simd_json::BorrowedValue;
use simd_json::prelude::ValueAsScalar;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

fn main() -> Result<()> {
	let file       = File::open("data.json")?;                       // Open the JSON file
	let mut reader = BufReader::new(file);
	let e_no_array_start = io::Error::new(io::ErrorKind::UnexpectedEof, "Reached EOF before finding '['");

    // 1) Skip whitespace until we see the initial '['
    {
        let mut buf = [0u8; 1];
        loop 
        {
            let n = reader.read(&mut buf)?;
            if n == 0         { return Err(e_no_array_start) }
            if buf[0] == b'[' { break; } 
            else if buf[0].is_ascii_whitespace() { continue; } 
            else 
            {
					let msg = format!( "Expected '[' at the start of JSON array, found byte 0x{:02X}", buf[0]);
					let err = io::Error::new(io::ErrorKind::InvalidData, msg);
					return Err(err)
            }
        }
    }

    // 2) Now stream‐read each JSON object until we hit the closing ']'.
    //
    //    We track braces so we know where each object ends. We also skip any commas
    //    separating objects. Each complete `{ ... }` is handed to simd_json::to_borrowed_value
    //    as its own slice.
    let mut in_object   = false;
    let mut brace_count = 0usize;
    let mut obj_buf     = Vec::new();

    for byte_result in reader.bytes() {
        let b = byte_result?;

        if !in_object {
            // We are not currently accumulating an object.
            // Skip whitespace and commas until we see '{' or ']'.
            if b.is_ascii_whitespace() || b == b',' { continue; }
            if b == b']' { // End of the top‐level array → stop streaming break; 
            	break;
            }
            if b == b'{' {
                // Start a new object
                in_object = true;
                brace_count = 1;
                obj_buf.clear();
                obj_buf.push(b);
            } else {
                // Unexpected character outside an object
                continue;
            }
        } else {
            // We are inside an object; accumulate bytes until brace_count reaches zero.
            obj_buf.push(b);
            if b == b'{' {
                brace_count += 1;
            } else if b == b'}' {
                brace_count -= 1;
                if brace_count == 0 {
                    // We have a complete `{ ... }` in obj_buf.
                    // 3) Parse it with simd_json (in‐place).
                    let mut temp = obj_buf.clone();
                    match simd_json::to_borrowed_value(&mut temp) {
                        Ok(BorrowedValue::Object(map)) => {
                            // Handle the object as needed. For example:
                            let city     = map.get("City") .and_then(|v| v.as_str()).unwrap_or("<missing City>");
                            let temp_val = map.get("Temperature") .and_then(|v| v.as_f64()).unwrap_or(f64::NAN);
                            println!("City = \"{city}\", Temperature = {temp_val}");
                        }
                        Ok(other) => { eprintln!("Expected object, got: {other:#?}"); }
                        Err(e)   => { eprintln!("JSON parse error in object: {}", e); }
                    }
                    in_object = false;
                }
            }
        }
    }

    Ok(())
}