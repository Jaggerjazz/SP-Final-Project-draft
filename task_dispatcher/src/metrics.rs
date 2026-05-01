use crate::task::Task;
use std::time::{Duration, Instant};

pub struct Metrics {
    pub total_completed: usize,
    pub total_wait_time: Duration,
    pub total_turnaround: Duration,
    pub total_busy_time: Duration,
    pub max_wait_time: Duration,
    pub max_queue_length: usize,
    pub worker_count: usize,
    pub start_time: Instant,
}

impl Metrics {
    pub fn new(worker_count: usize) -> Self {
        Self {
            total_completed: 0,
            total_wait_time: Duration::ZERO,
            total_turnaround: Duration::ZERO,
            total_busy_time: Duration::ZERO,
            max_wait_time: Duration::ZERO,
            max_queue_length: 0,
            worker_count,
            start_time: Instant::now(),
        }
    }

    pub fn record(&mut self, task: Task, start: Instant, finish: Instant) {
        self.total_completed += 1;

        let wait = start.duration_since(task.arrival_time);
        let turnaround = finish.duration_since(task.arrival_time);
        let busy_time = finish.duration_since(start);

        self.total_wait_time += wait;
        self.total_turnaround += turnaround;
        self.total_busy_time += busy_time;

        if wait > self.max_wait_time {
            self.max_wait_time = wait;
        }
    }

    pub fn observe_queue_length(&mut self, queue_length: usize) {
        if queue_length > self.max_queue_length {
            self.max_queue_length = queue_length;
        }
    }

    pub fn average_wait_time(&self) -> Duration {
        if self.total_completed == 0 {
            Duration::ZERO
        } else {
            self.total_wait_time / self.total_completed as u32
        }
    }

    pub fn average_turnaround_time(&self) -> Duration {
        if self.total_completed == 0 {
            Duration::ZERO
        } else {
            self.total_turnaround / self.total_completed as u32
        }
    }

    pub fn worker_utilization(&self) -> f64 {
        let makespan = self.makespan().as_secs_f64();
        if makespan == 0.0 || self.worker_count == 0 {
            0.0
        } else {
            self.total_busy_time.as_secs_f64() / (makespan * self.worker_count as f64)
        }
    }

    pub fn makespan(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn print(&self) {
        println!("Total completed: {}", self.total_completed);

        println!("Avg wait: {:?}", self.average_wait_time());
        println!("Max wait: {:?}", self.max_wait_time);
        println!("Avg turnaround: {:?}", self.average_turnaround_time());
        println!("Peak queue length: {}", self.max_queue_length);
        println!("Worker utilization: {:.2}%", self.worker_utilization() * 100.0);
        println!("Makespan: {:?}", self.makespan());
    }
}