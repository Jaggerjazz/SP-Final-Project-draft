use crate::metrics::Metrics;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn report(
    experiment_name: &str,
    total_tasks: usize,
    worker_count: usize,
    cpu_probability: f64,
    seed: u64,
    results_file: &str,
    metrics: &Metrics,
) -> io::Result<()> {
    let report = format!(
        "\n=== {} ===\nConfig: tasks={}, workers={}, cpu_probability={:.2}, seed={}\n{}",
        experiment_name,
        total_tasks,
        worker_count,
        cpu_probability,
        seed,
        metrics.render()
    );

    print!("{}", report);

    let file_path = Path::new(results_file);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(file_path)?;
    file.write_all(report.as_bytes())?;

    Ok(())
}