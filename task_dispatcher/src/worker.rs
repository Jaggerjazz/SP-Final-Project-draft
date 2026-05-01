use crate::task::{TaskType, WorkerMessage};
use crate::metrics::Metrics;
use std::sync::{Arc, Mutex, mpsc::Receiver};
use std::thread;
use std::time::Instant;

pub fn start_workers(
    worker_count: usize,
    rx: Receiver<WorkerMessage>,
    metrics: Arc<Mutex<Metrics>>,
) -> Vec<thread::JoinHandle<()>> {
    let rx = Arc::new(Mutex::new(rx));
    let mut handles = Vec::with_capacity(worker_count);

    for _ in 0..worker_count {
        let rx = Arc::clone(&rx);
        let metrics = Arc::clone(&metrics);

        let handle = thread::spawn(move || loop {
            let message = {
                let guard = rx.lock().unwrap();
                guard.recv()
            };

            match message {
                Ok(WorkerMessage::Task(task)) => {
                    let start = Instant::now();
                    let _task_id = task.id;

                    match task.kind {
                        TaskType::CPU => {
                            let end = Instant::now() + task.duration;
                            while Instant::now() < end {}
                        }
                        TaskType::IO => {
                            thread::sleep(task.duration);
                        }
                    }

                    let finish = Instant::now();

                    let mut m = metrics.lock().unwrap();
                    m.record(task, start, finish);
                }
                Ok(WorkerMessage::Shutdown) | Err(_) => break,
            }
        });

        handles.push(handle);
    }

    handles
}