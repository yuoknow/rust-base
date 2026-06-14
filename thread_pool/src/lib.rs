#![deny(unsafe_code)]

use crate::sync::{Arc, Condvar, Mutex, thread};

#[cfg(feature = "loom")]
pub mod sync {
    pub use loom::sync::{Arc, Condvar, Mutex};
    pub use loom::thread;
}

#[cfg(not(feature = "loom"))]
pub mod sync {
    pub use std::sync::{Arc, Condvar, Mutex};
    pub use std::thread;
}

pub type Task = fn(i64);

struct Shared {
    state: Mutex<State>,
    has_work: Condvar,
}

struct State {
    queue: Vec<i64>,
    shutting_down: bool,
}

pub struct ThreadPool {
    shared: Arc<Shared>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a pool with `worker_count` workers.
    ///
    /// # Panics
    ///
    /// Should panic when `worker_count == 0`.
    pub fn new(worker_count: usize, task: Task) -> Self {
        if worker_count <= 0 {
            panic!("Wrong worker count")
        }

        let state = State {
            queue: Vec::new(),
            shutting_down: false,
        };

        let shared = Arc::new(Shared {
            state: Mutex::new(state),
            has_work: Condvar::new(),
        });

        let workers: Vec<thread::JoinHandle<()>> = (0..worker_count)
            .map(|_| {
                let shared_clone = Arc::clone(&shared);
                return thread::spawn(move || {
                    loop {
                        let maybe_num = {
                            let mut state = shared_clone.state.lock().expect("mutex poisoned");

                            while state.queue.is_empty() && !state.shutting_down {
                                state = shared_clone
                                    .has_work
                                    .wait(state)
                                    .expect("mutex poisoned while waiting");
                            }

                            if let Some(num) = state.queue.pop() {
                                Some(num)
                            } else if state.shutting_down {
                                None
                            } else {
                                continue;
                            }
                        };

                        match maybe_num {
                            Some(num) => task(num),
                            None => break,
                        }
                    }
                });
            })
            .collect();

        ThreadPool { shared, workers }
    }

    /// Add one number to the work queue.
    pub fn execute(&self, num: i64) {
        let mut state = self.shared.state.lock().expect("mutex poisoned");
        state.queue.push(num);
        self.shared.has_work.notify_one();
    }

    /// Finish all queued work and stop all workers.
    pub fn shutdown(self) {
        let mut state = self.shared.state.lock().expect("mutex poisoned");
        state.shutting_down = true;
        self.shared.has_work.notify_all();
        drop(state);
        self.workers
            .into_iter()
            .for_each(|worker| worker.join().expect("worker thread panicked"));
    }
}
