
#[derive(Clone, Copy, Debug, Default)]
pub enum ComponentStatus{
    #[default]
    Idle,
    Active,
    Terminated,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ComponentMetrics{
    pub id:                       u32,
    pub status:       ComponentStatus,
    pub duration_ms:              u64,
    pub byte_count:               u64,
    pub record_count:             u64,
    pub error_count:              u64,
}


impl ComponentMetrics {
	pub fn new(id: u32) -> Self {
		let status = ComponentStatus::Idle;
		ComponentMetrics{
			id, 
			status,
			duration_ms:    0, 
			byte_count:     0, 
			record_count:   0,
			error_count:    0,
		}
	}
}