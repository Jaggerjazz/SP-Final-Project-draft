use crate::task::{Task, WorkerMessage};
use crate::metrics::Metrics;
use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_dispatcher(
    rx: Receiver<Task>,
    worker_tx: Sender<WorkerMessage>,
    worker_count: usize,
    metrics: Arc<Mutex<Metrics>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut queue: VecDeque<Task> = VecDeque::new();
        let mut input_closed = false;

        loop {
            while let Ok(task) = rx.try_recv() {
                queue.push_back(task);
                metrics.lock().unwrap().observe_queue_length(queue.len());
            }

            if let Some(task) = queue.pop_front() {
                if worker_tx.send(WorkerMessage::Task(task)).is_err() {
                    return;
                }

                continue;
            }

            if input_closed {
                break;
            }

            match rx.recv_timeout(Duration::from_millis(5)) {
                Ok(task) => {
                    queue.push_back(task);
                    metrics.lock().unwrap().observe_queue_length(queue.len());
                }
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => input_closed = true,
            }
        }

        for _ in 0..worker_count {
            if worker_tx.send(WorkerMessage::Shutdown).is_err() {
                break;
            }
        }
    })
}