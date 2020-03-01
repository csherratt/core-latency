extern crate core_affinity;
extern crate crossbeam;

use core_affinity::CoreId;
use std::time::{Instant, Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam::utils::CachePadded;
use std::thread::spawn;

const REPS_PER_ROUND: usize = 8192;
const ROUNDS: usize = 1024;

fn spawn_thread(a: CoreId, mut ping: Arc<CachePadded<AtomicUsize>>, mut pong: Arc<CachePadded<AtomicUsize>>) -> Vec<Duration> {
    core_affinity::set_for_current(a);
    let mut results = Vec::new();
    results.reserve(ROUNDS);

    let mut current = 0;
    for i in 0..ROUNDS {
        let start = Instant::now();
        for j in 0..REPS_PER_ROUND {
            current += 1;
            ping.store(current, Ordering::Relaxed);
            while pong.load(Ordering::Relaxed) != current {
            }
            std::mem::swap(&mut ping, &mut pong);
        }
        let end = Instant::now();
        results.push(end.duration_since(start));
    }

    results
}

fn spawn_and_measure(a: CoreId, b: CoreId) -> Duration {
    let ping = Arc::new(CachePadded::new(AtomicUsize::new(0)));
    let pong = Arc::new(CachePadded::new(AtomicUsize::new(0)));

    let (aping, apong) = (pong.clone(), ping.clone());
    let handle_a = spawn(move || {
        spawn_thread(a, aping, apong)
    });
    let handle_b = spawn(move || {
        spawn_thread(b, ping, pong)
    });

    let mut x = handle_a.join().ok().unwrap();
    x.append(&mut handle_b.join().ok().unwrap());
    x.sort();
    x[x.len()/2]
}

fn main() {
    // Retrieve the IDs of all active CPU cores.
    let core_ids = core_affinity::get_core_ids().unwrap();

    println!("Core count {}", core_ids.len());

    for a in &core_ids {
        let mut nums = Vec::new();
        for b in &core_ids {
            if a.id == b.id {
                nums.push(0);
                continue
            }
            nums.push(spawn_and_measure(*a, *b).as_nanos());
        }
        let nums: Vec<_> = nums.into_iter().map(|duration| format!("{:3.1}", duration as f64 / (REPS_PER_ROUND as f64))).collect();
        println!("{}", nums.join(","));
    }
}
