use crate::metrics::Metrics;

pub fn report(
    experiment_name: &str,
    total_tasks: usize,
    worker_count: usize,
    cpu_probability: f64,
    seed: u64,
    metrics: &Metrics,
) {
    println!("\n=== {} ===", experiment_name);
    println!("Config: tasks={}, workers={}, cpu_probability={:.2}, seed={}", total_tasks, worker_count, cpu_probability, seed);
    metrics.print();
}