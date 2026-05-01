# Concurrent Task Dispatcher in Rust

A Rust simulation of a concurrent task dispatcher with a generator, a central queue-based dispatcher, and a bounded worker pool.

## How to build and run

```bash
cd task_dispatcher
cargo build
cargo run
```

To check that it compiles without running the simulation:

```bash
cd task_dispatcher
cargo check
```

## Project summary

The program simulates tasks arriving over time, placing them into a dispatcher queue and sending them to a fixed pool of eight workers.

Each task contains:
- `id`
- `arrival_time`
- `kind` (`CPU` or `IO`)
- `duration`

The simulation runs two experiments automatically:
- Balanced workload: 50/50 CPU and IO
- CPU-heavy workload: 80/20 CPU and IO

Both experiments use fixed random seeds so the task mix is reproducible.

## Design summary

### Components
- `generator.rs` creates tasks and simulates arrival over time.
- `dispatcher.rs` owns the ready queue and forwards tasks to workers.
- `worker.rs` runs the bounded worker pool and simulates CPU or IO execution.
- `metrics.rs` collects summary statistics.
- `monitor.rs` prints the final report for each experiment.
- `task.rs` defines the task model and shutdown messages.

### Concurrency model
- A generator thread produces work.
- A dispatcher thread queues and forwards work.
- Eight worker threads execute tasks concurrently.
- Shared metrics are protected with `Arc<Mutex<_>>`.
- Channels are used for task handoff and shutdown signaling.

### Scheduling policy
The dispatcher uses FIFO queueing.
That is a real scheduling policy because tasks wait in a queue before execution, and the dispatcher decides the next task in arrival order.

## Metrics collected

Required metrics:
- total tasks completed
- makespan
- average wait time
- average turnaround time

Additional metrics:
- worker utilization
- peak queue length
- max wait time

## Experiments

The program runs two experiments back-to-back:

1. Balanced workload
- 50/50 CPU and IO tasks
- seed: 42
- 1000 tasks
- 8 workers

2. CPU-heavy workload
- 80/20 CPU and IO tasks
- seed: 99
- 1000 tasks
- 8 workers

Expected interpretation:
- The balanced workload should keep the queue shorter and reduce waiting.
- The CPU-heavy workload should increase wait time, queue length, and makespan.

The exact timings depend on the machine running the simulation, but the comparison is reproducible.

## Clean shutdown

The program does not rely on `sleep()` to guess when it should stop.
The generator finishes and closes its channel, the dispatcher drains the queue and sends shutdown messages, and the workers exit cleanly.

## Tool Use Disclosure

I used GitHub Copilot and the VS Code workspace tools while building this project.
They helped with code structure, concurrency cleanup, and documentation drafting.

Example of advice I accepted:
- replace the `sleep()`-based shutdown with channel closing and explicit shutdown messages

Example of advice I had to fix:
- the first queue-tracking pass was too loose; I updated it to record queue length from a real dispatcher queue
