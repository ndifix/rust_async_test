use std::{collections::VecDeque, sync::{atomic::AtomicU64, Mutex}, thread};

fn main() {
    let queue: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
    let max = 10;
    let count = AtomicU64::new(max);

    thread::scope(|s| {
        /*
         * Get and print data from queue
         */
        let handle = s.spawn(|| {
            loop {
                if count.load(std::sync::atomic::Ordering::Relaxed) == 0 {
                    return;
                }

                let data = queue.lock().unwrap().pop_front();
                if let Some(data) = data {
                    println!("{}", data);
                    count.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                } else {
                    thread::park();
                }
            }
        });

        /*
         * Generate data and push to queue
         */
        for i in 0..max {
            let data = gen_data(i);
            queue.lock().unwrap().push_back(data);
            handle.thread().unpark();
        }
        println!("done pushing");
    });
}

fn gen_data(t: u64) -> String {
    format!("generated {}", t)
}
