#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParseStatus {
	NotParsed,
	InProgress,
	Finished,
	Error,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParseDetailDTO {
	file_name:          String,
	parse_status:       ParseStatus,
	bytes_parsed:       u64,
	parse_duration_ms:  u32,
	data_rows:          u64,
	errors_encountered: u64,
}

impl ParseDetailDTO {
	pub fn new(file_name: &str) -> Self {
	ParseDetailDTO {
			file_name:          file_name.to_string(),
			parse_status:       ParseStatus::NotParsed,
			bytes_parsed:       0,
			parse_duration_ms:  0,
			data_rows:          0,
			errors_encountered: 0,
		}
	}
	
    /* ───────────── getters ───────────── */
    /// Original file name (borrowed).
    #[inline]
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Current parse status enum.
    #[inline]
    pub fn parse_status(&self) -> ParseStatus {
        self.parse_status
    }

    /// Total bytes that have been processed so far.
    #[inline]
    pub fn bytes_parsed(&self) -> u64 {
        self.bytes_parsed
    }

    /// Time spent parsing (milliseconds).
    #[inline]
    pub fn parse_duration_ms(&self) -> u32 {
        self.parse_duration_ms
    }

    /// Number of data rows successfully read.
    #[inline]
    pub fn data_rows(&self) -> u64 {
        self.data_rows
    }

    /// Number of non-fatal errors encountered while parsing.
    #[inline]
    pub fn errors_encountered(&self) -> u64 {
        self.errors_encountered
    }    
    
    /// Replace all numeric counters in one call and return the updated value.
    ///
    /// Useful for code that collects all metrics first and then does a single
    /// immutable update:
    ///
    /// ```rust
    /// dto = dto.with_metrics(total_bytes, elapsed_ms, rows, errs);
    /// ```
    /// ```rust
    /// dto = dto
    ///     .with_metrics(b, t, rows, errs)
    ///     .finished()
    /// ```
    #[inline]
    pub fn with_metrics(
        self,
        bytes_parsed:       u64,
        parse_duration_ms:  u32,
        data_rows:          u64,
        errors_encountered: u64,
    ) -> Self {
        Self {
            bytes_parsed,
            parse_duration_ms,
            data_rows,
            errors_encountered,
            // keep the other fields the same
            ..self
        }
    }    

    /// Return a **new** value with `parse_status` replaced.
    #[inline]
    pub fn with_parse_status(self, status: ParseStatus) -> Self {
        Self {
            parse_status: status,
            ..self
        }
    }

    /// Convenience wrapper that marks the record **finished**.
    /// 
    #[inline]
    pub fn finished(&self) -> Self {
        Self {
            parse_status: ParseStatus::Finished,
            ..self.clone()
        }
    }    
}

