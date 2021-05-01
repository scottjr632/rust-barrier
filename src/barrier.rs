use std::sync::atomic::{AtomicUsize,Ordering,AtomicBool};

pub struct Barrier {
    initial_count: usize,
    count: AtomicUsize,
    sense: AtomicBool,
}

impl Barrier {
    pub fn init(initial_count: usize) -> Barrier {
        Barrier {
            initial_count,
            count: AtomicUsize::new(initial_count),
            sense: AtomicBool::new(true),
        }
    }
    pub fn arrive(&self) {
        let priv_sense = self.sense.load(Ordering::SeqCst);
        if self.count.fetch_sub(1, Ordering::SeqCst) == 1 {
            self.count.store(self.initial_count, Ordering::SeqCst);
            self.sense.store(!priv_sense, Ordering::SeqCst);
        } else {
            while priv_sense == self.sense.load(Ordering::SeqCst) {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Instant};
    use std::thread;
    use std::thread::JoinHandle;
    use std::sync::Arc;

    static ITERS: u128 = 10000000;

    #[test]
    fn bench_it_works() {
        let mut threads: Vec<JoinHandle<()>> = Vec::new();
        let num_threads = 4;
        let b = Arc::new(Barrier::init(num_threads));
        let now = Instant::now();
        for _ in 0..num_threads {
            let b_clone = Arc::clone(&b);
            threads.push(
                thread::spawn(move || {
                    for _ in 0..ITERS {
                        b_clone.arrive();
                    }
                })
            )
        }
    
        for t in threads {
            t.join().unwrap();
        }
        println!("ran for {}ns", now.elapsed().as_nanos() / ITERS);
        assert_eq!(4, 4);
    }

}