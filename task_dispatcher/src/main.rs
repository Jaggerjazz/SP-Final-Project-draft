mod task;
mod generator;
mod dispatcher;
mod worker;
mod metrics;
mod monitor;

use std::sync::{Arc, Mutex, mpsc};
use metrics::Metrics;
use task::WorkerMessage;

struct ExperimentSpec {
    name: &'static str,
    cpu_probability: f64,
    seed: u64,
    total_tasks: usize,
    results_file: &'static str,
}

fn main() {
    run_experiment(ExperimentSpec {
        name: "Balanced workload (50/50 CPU/IO)",
        cpu_probability: 0.5,
        seed: 42,
        total_tasks: 1000,
        results_file: "results/balance.txt",
    });

    run_experiment(ExperimentSpec {
        name: "CPU-heavy workload (80/20)",
        cpu_probability: 0.8,
        seed: 99,
        total_tasks: 1000,
        results_file: "results/cpu.txt",
    });
}

fn run_experiment(spec: ExperimentSpec) {
    let worker_count = 8;

    let (gen_tx, gen_rx) = mpsc::channel();
    let (worker_tx, worker_rx) = mpsc::channel::<WorkerMessage>();

    let metrics = Arc::new(Mutex::new(Metrics::new(worker_count)));

    let generator_handle = generator::start_generator(
        gen_tx,
        spec.total_tasks,
        spec.cpu_probability,
        spec.seed,
    );
    let dispatcher_handle = dispatcher::start_dispatcher(
        gen_rx,
        worker_tx,
        worker_count,
        Arc::clone(&metrics),
    );
    let worker_handles = worker::start_workers(worker_count, worker_rx, Arc::clone(&metrics));

    let _ = generator_handle.join();
    let _ = dispatcher_handle.join();

    for handle in worker_handles {
        let _ = handle.join();
    }

    let m = metrics.lock().unwrap();
    monitor::report(spec.name, spec.total_tasks, worker_count, spec.cpu_probability, spec.seed, spec.results_file, &m)
        .expect("failed to write experiment results");
}