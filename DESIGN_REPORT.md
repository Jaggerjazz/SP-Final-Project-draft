# Design Report: Concurrent Task Dispatcher in Rust

## 1. Architecture

This project is a simulation of an operating-system-style task dispatcher.
The code is split into a few focused pieces:

- `main.rs` configures each experiment and coordinates startup/shutdown.
- `generator.rs` creates synthetic tasks over time.
- `dispatcher.rs` owns the ready queue and forwards tasks to workers.
- `worker.rs` starts a bounded worker pool and executes the simulated work.
- `metrics.rs` tracks timing and queue statistics.
- `monitor.rs` prints the final results for each experiment.
- `task.rs` defines the task model and worker control messages.

The system is intentionally simple: one generator thread, one dispatcher thread, and a fixed pool of eight workers.
That structure makes it easy to explain where a task is at each step.

## 2. Task model

Each task carries the required fields:

- `id`
- `arrival_time`
- `kind` (`CPU` or `IO`)
- `duration`

The generator creates tasks with a fixed seed so the workload pattern is reproducible.
That matters because the project is a simulation: the workload should be comparable across runs.

## 3. Queue and scheduling design

The dispatcher uses a FIFO ready queue.
Tasks are inserted when they arrive and removed in arrival order.
This is a real scheduling policy, not just a pass-through, because the queue can grow when arrivals outpace service.

Why FIFO:
- It is easy to explain.
- It is fair among same-priority tasks.
- It gives a clear baseline for comparing workload behavior.

Trade-off:
- FIFO can create head-of-line blocking.
- Long CPU-style tasks can delay shorter IO-style tasks.
- Under a CPU-heavy workload, the queue can grow faster and wait times can increase.

## 4. Synchronization strategy

The project uses standard Rust concurrency primitives:

- `std::thread` for concurrent execution
- `std::sync::mpsc` channels for task handoff and shutdown signaling
- `Arc<Mutex<Metrics>>` for shared statistics

Why channels:
- The generator and dispatcher communicate through a channel because ownership should transfer cleanly.
- The dispatcher and workers also use a channel so the worker pool can block waiting for work instead of busy looping.

Why shared state:
- Metrics must be updated by workers as tasks finish.
- `Arc<Mutex<_>>` is appropriate because the data is small, shared, and updated frequently but briefly.

## 5. Clean shutdown

A bug I hit during development was relying on a `sleep()` call to keep the program alive long enough.
That is fragile because the program may sleep too little or too long.

The fix was:
- let the generator finish and close its sender
- let the dispatcher detect the closed input channel
- send explicit `Shutdown` messages to workers
- join all worker threads before printing the summary

That makes shutdown deterministic and avoids leaving threads blocked forever.

## 6. Metrics collected

Required metrics:

- total tasks completed
- makespan
- average wait time
- average turnaround time

Additional metrics:

- worker utilization
- peak queue length
- max wait time

Why these help:
- worker utilization shows whether the worker pool is being used effectively
- peak queue length shows how much backlog the dispatcher experienced
- max wait time exposes worst-case delay, which average wait time can hide

## 7. Experiments

The program runs two experiments automatically:

### Experiment A: balanced workload
- 50/50 CPU and IO tasks
- fixed seed `42`
- 1000 tasks
- 8 workers

### Experiment B: stressed workload
- 80/20 CPU and IO tasks
- fixed seed `99`
- 1000 tasks
- 8 workers

### Expected comparison
The balanced workload should be easier for the dispatcher to absorb.
The CPU-heavy workload should stress the queue more because more tasks spend time in CPU simulation.
That should increase:
- queue length
- average wait time
- max wait time
- makespan

On a local run, both experiments complete successfully and print a full summary for comparison.
The exact timing values vary by machine, but the relative trend is the important part.

## 8. What improved and what got worse

Improved by this design:
- the program now has a clean, explainable producer -> queue -> worker flow
- shutdown is deterministic
- metrics expose the scheduling behavior instead of hiding it

What is still limited:
- FIFO is simple, but it can be unfair to short tasks when long CPU tasks build up
- one central queue can become a bottleneck if the workload grows much larger
- the worker pool is fixed-size, so it does not adapt to load

## 9. Potential starvation or unfairness

Starvation is not the main failure mode here, but unfairness can still happen.
A burst of CPU-heavy work can make IO-style tasks wait longer than expected.
Because the queue is FIFO and there is only one dispatcher queue, tasks are not prioritized by length or class.
A smarter policy such as aging, weighted CPU/IO dispatch, or shortest-job-first could reduce that imbalance, but at the cost of more complexity.

## 10. Lessons learned

The project is small, but it captures the core concurrency ideas well:

- ownership moves through channels
- shared state should stay small and synchronized carefully
- shutdown needs explicit design
- a policy is only useful if it can be explained and measured

The final system is not a real operating system, but it behaves like a useful scheduler simulation and is straightforward to defend during a demo.
