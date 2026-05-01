use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub enum TaskType {
    CPU,
    IO,
}

#[derive(Clone, Debug)]
pub struct Task {
    pub id: usize,
    pub arrival_time: Instant,
    pub kind: TaskType,
    pub duration: Duration,
}

#[derive(Clone, Debug)]
pub enum WorkerMessage {
    Task(Task),
    Shutdown,
}