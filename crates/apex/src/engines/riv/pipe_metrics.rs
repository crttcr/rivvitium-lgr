
#[derive(Clone, Copy, Debug, Default)]
pub enum ComponentStatus{
    #[default]
    Idle,
    Active,
    Terminated,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PipeMetrics{
    pub component_id:      u16,
    pub component_status : ComponentStatus,
    pub duration_ms :      u64,
    pub byte_count  :      u64,
    pub record_count:      u64,
}
