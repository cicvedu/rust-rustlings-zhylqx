// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.


use std::thread;
use std::time::{Duration, Instant};

fn main() {

    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
        }));
    }

    let mut completed_threads = 0;
    for handle in handles {
        // A struct is returned from thread::spawn, can you use it?
        handle.join(); // Yes, you can.
        completed_threads += 1;
    }

    if completed_threads != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }
    
}