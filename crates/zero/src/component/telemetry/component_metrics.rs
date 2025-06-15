
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ComponentStatus{
    #[default]
    Idle,
    Active,
    Completed,
    Failed,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ComponentMetrics{
    pub id:               u32,
    pub status:           ComponentStatus,
    pub duration:         Duration,
    pub message_count:    u64,
    pub byte_count:       u64,
    pub record_count:     u64,
    pub error_count:      u64,
}


impl ComponentMetrics {
	pub fn new(id: u32) -> Self {
		ComponentMetrics{
			id, 
			status:         ComponentStatus::Idle,
			duration:       Duration::ZERO,
			message_count:  0,
			byte_count:     0, 
			record_count:   0,
			error_count:    0,
		}
	}
}
	
// State management
impl ComponentMetrics {
	/// Marks the component as active.
	pub fn activate(&mut self) {
		 self.status = ComponentStatus::Active;
	}

	/// Marks the component as successfully completed.
	pub fn complete(&mut self) {
		 self.status = ComponentStatus::Completed;
	}

	/// Marks the component as failed.
	pub fn fail(&mut self) {
		 self.status = ComponentStatus::Failed;
	}
}

// Fluent updates
impl ComponentMetrics {
	/// Resets all counters
	pub fn reset(&mut self) -> &mut Self {
		self.duration        = Duration::ZERO;
		self.message_count   = 0;
		self.byte_count      = 0;
		self.record_count    = 0;
		self.error_count     = 0;
		self
	}

	/// Sets the total duration for the component.
	pub fn set_duration(&mut self, duration: Duration) -> &mut Self {
		 self.duration = duration;
		 self
	}

	/// Adds to the total byte count.
	pub fn add_bytes(&mut self, bytes: u64) -> &mut Self {
		 self.byte_count += bytes;
		 self
	}

	/// Adds to the total record count.
	pub fn increment_messages(&mut self) -> &mut Self {
		 self.message_count += 1;
		 self
	}

	/// Adds to the total record count.
	pub fn add_records(&mut self, records: u64) -> &mut Self {
		 self.record_count += records;
		 self
	}

	/// Increments the error count by one.
	pub fn increment_errors(&mut self) -> &mut Self {
		 self.error_count += 1;
		 self
	}
}

// Calculated values
//
impl ComponentMetrics {
	/// Calculates the processing rate in records per second.
	/// Returns `None` if the duration was zero to avoid division by zero.
	pub fn messages_per_second(&self) -> Option<f64> {
		 let seconds = self.duration.as_secs_f64();
		 if seconds > 0.0 && self.message_count > 0 {
			  Some(self.message_count as f64 / seconds)
		 } else {
			  None
		 }
	}

	/// Calculates the processing rate in records per second.
	/// Returns `None` if the duration was zero to avoid division by zero.
	pub fn records_per_second(&self) -> Option<f64> {
		 let seconds = self.duration.as_secs_f64();
		 if seconds > 0.0 && self.record_count > 0 {
			  Some(self.record_count as f64 / seconds)
		 } else {
			  None
		 }
	}

	/// Calculates data throughput in megabytes per second (MB/s).
	/// Returns `None` if the duration was zero.
	pub fn throughput_mb_per_sec(&self) -> Option<f64> {
		 let seconds = self.duration.as_secs_f64();
		 if seconds > 0.0 && self.byte_count > 0 {
			  let megabytes = self.byte_count as f64 / (1024.0 * 1024.0);
			  Some(megabytes / seconds)
		 } else {
			  None
		 }
	}

	/// Calculates the error rate as a ratio of errors to total records.
	/// Returns `None` if no records were processed.
	pub fn error_rate(&self) -> Option<f64> {
		 if self.record_count == 0 {
			  None
		 } else {
			  Some(self.error_count as f64 / self.record_count as f64)
		 }
	}

	/// Returns the byte_count as a human-readable string (e.g., "1.23 MiB").
	pub fn human_readable_bytes(&self) -> String {
		 const KIB: f64 = 1024.0;
		 const MIB: f64 = KIB * 1024.0;
		 const GIB: f64 = MIB * 1024.0;
		 
		 let bytes = self.byte_count as f64;

		 if      bytes < KIB { format!("{} B", bytes) } 
		 else if bytes < MIB { format!("{:.2} KiB", bytes / KIB) } 
		 else if bytes < GIB { format!("{:.2} MiB", bytes / MIB) } 
		 else                { format!("{:.2} GiB", bytes / GIB) }
	}
}


use std::ops::{Add, AddAssign};

impl Add for ComponentMetrics {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Define a priority for setting the combined status
        let status = match (self.status, rhs.status) {
            (ComponentStatus::Failed, _) | (_, ComponentStatus::Failed) => ComponentStatus::Failed,
            (ComponentStatus::Active, _) | (_, ComponentStatus::Active) => ComponentStatus::Active,
            _ => ComponentStatus::Completed, // Otherwise, consider the aggregation 'Completed'
        };

        Self {
            id: 0, // An ID of 0 can signify an aggregate
            status,
            duration:       self.duration      + rhs.duration,
            message_count:  self.message_count + rhs.message_count,
            byte_count:     self.byte_count    + rhs.byte_count,
            record_count:   self.record_count  + rhs.record_count,
            error_count:    self.error_count   + rhs.error_count,
        }
    }
}


impl AddAssign for ComponentMetrics {
    fn add_assign(&mut self, rhs: Self) {
        // When adding in-place, we keep the original ID.
        self.status = match (self.status, rhs.status) {
            (ComponentStatus::Failed, _) | (_, ComponentStatus::Failed) => ComponentStatus::Failed,
            (ComponentStatus::Active, _) | (_, ComponentStatus::Active) => ComponentStatus::Active,
            _ => ComponentStatus::Completed,
        };
        self.duration        += rhs.duration;
        self.message_count   += rhs.message_count;
        self.byte_count      += rhs.byte_count;
        self.record_count    += rhs.record_count;
        self.error_count     += rhs.error_count;
    }
}