use std::thread;
use std::sync::{Arc, Mutex};


fn main() -> std::io::Result<()> {
    
    // We create an Arc (Atomic Reference Counted) to allow multiple threads to share ownership of the counter variable
    let counter = Arc::new(Mutex::new(0));
    // We create a vector to hold the thread handles so we can join them later
    let mut handles = vec![];

    // We spawn 10 threads, each of which will increment the counter 100 times
    calculation(&counter,&mut handles); // We call the calculation function to perform some work before starting the threads
    // for _ in 0..10 {
    //     // We clone the Arc to get a new reference to the counter for each thread
    //     let counter_clone = Arc::clone(&counter);
    //     // We spawn a new thread and move the cloned Arc into the thread's closure
    //     let handle = thread::spawn(move || {
    //         for _ in 0..100 {
    //             // We lock the mutex to get mutable access to the counter, increment it, and then the lock is automatically released when the scope ends
    //             let mut num = counter_clone.lock().unwrap();
    //             // We increment the counter, deferencing the mutex guard to get the actual value of the counter and then incrementing it
    //             *num += 1;
    //         }
    //     });
    //     // We push the thread handle into the vector so we can join it later
    //     handles.push(handle);
    // }

    // We join all the threads to ensure they have completed before we print the final counter value
    for handle in handles {
        // We call join on each thread handle to wait for the thread to finish executing
        handle.join().unwrap();
    }

    // Finally, we print the final value of the counter, which should be 1000 (10 threads * 100 increments each)
    println!("Final counter value: {}", *counter.lock().unwrap());

    Ok(())
}

fn calculation(counter: &Arc<Mutex<i32>>, handles: &mut Vec<thread::JoinHandle<()>>) {
    for _ in 0..10 {
        // We clone the Arc to get a new reference to the counter for each thread
        let counter_clone = Arc::clone(counter);
        // We spawn a new thread and move the cloned Arc into the thread's closure
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                // We lock the mutex to get mutable access to the counter, increment it, and then the lock is automatically released when the scope ends
                let mut num = counter_clone.lock().unwrap();
                // We increment the counter, deferencing the mutex guard to get the actual value of the counter and then incrementing it
                *num += 1;
            }
        });
        // We push the thread handle into the vector so we can join it later
        handles.push(handle);
    }
}
