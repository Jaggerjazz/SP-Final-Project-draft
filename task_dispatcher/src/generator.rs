use crate::task::{Task, TaskType};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};

pub fn start_generator(
    tx: Sender<Task>,
    total: usize,
    cpu_probability: f64,
    seed: u64,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut rng = StdRng::seed_from_u64(seed);

        for id in 0..total {
            let kind = if rng.gen_bool(cpu_probability) {
                TaskType::CPU
            } else {
                TaskType::IO
            };

            let duration = Duration::from_millis(200);

            let task = Task {
                id,
                arrival_time: Instant::now(),
                kind,
                duration,
            };

            tx.send(task).unwrap();

            thread::sleep(Duration::from_millis(20)); // simulate arrival rate
        }
    })
}