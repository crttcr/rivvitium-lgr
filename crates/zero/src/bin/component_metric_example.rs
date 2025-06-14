use std::time::{Duration, Instant};
use zero::component::telemetry::component::ComponentMetrics;

fn main() {
    // --- Individual Component Simulation ---
    let start_time = Instant::now();
    let mut ingest_metrics = ComponentMetrics::new(101);
    ingest_metrics.activate();

    // Simulate work...
    std::thread::sleep(Duration::from_millis(150));
    ingest_metrics.add_records(5000).add_bytes(2_500_000);
    ingest_metrics.increment_errors();
    ingest_metrics.increment_errors();

    ingest_metrics.set_duration(start_time.elapsed());
    ingest_metrics.complete();

    println!("--- Ingest Component (ID: {}) ---",        ingest_metrics.id);
    println!("Status              : {:?}",               ingest_metrics.status);
    println!("Processing Time     : {:.2?}",             ingest_metrics.duration);
    println!("Data Processed      : {}",                 ingest_metrics.human_readable_bytes());
    println!("Error Rate          : {:.2}%",             ingest_metrics.error_rate().unwrap_or(0.0) * 100.0);
    println!("Throughput          : {:.2} records/sec",  ingest_metrics.records_per_second().unwrap_or(0.0));
    println!();

    // --- Aggregation Example ---
    let mut processing_metrics = ComponentMetrics::new(202);
    processing_metrics.set_duration(Duration::from_millis(220))
                      .add_records(5000)
                      .add_bytes(1_000_000)
                      .complete();

    // Use the `+` operator to get a total
    let total_metrics = ingest_metrics + processing_metrics;

    println!("--- Aggregated Pipeline Metrics ---");
    println!("Combined Status    : {:?}",       total_metrics.status);
    println!("Total Time         : {:.2?}",     total_metrics.duration);
    println!("Total Records      : {}",         total_metrics.record_count);
    println!("Total Data         : {}",         total_metrics.human_readable_bytes());
    println!("Overall Throughput : {:.2} MB/s", total_metrics.throughput_mb_per_sec().unwrap_or(0.0));
}
