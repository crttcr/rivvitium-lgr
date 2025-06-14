
use crate::component::telemetry::component::ComponentMetrics;

pub trait ProvidesMetrics {
    /// Returns a non-destructive snapshot of the component's current metrics.
    fn metrics(&self) -> ComponentMetrics;

    /// Returns the current metrics and resets the component's internal counters.
    ///
    /// This is a destructive operation intended for periodic metric collection.
    fn take_metrics(&mut self) -> ComponentMetrics;
}