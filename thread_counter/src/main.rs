/*
Threaded Counter: Create 10 threads that each increment a shared counter 
(wrapped in Arc<Mutex<i32>>) 1,000 times. Print the final value (should be 10,000). 
Introduce a deliberate race condition first (without mutex) to see errors, then fix it. 
Learn: Race conditions, mutex usage, and Arc for shared ownership.
*/

fn main() {
    println!("Hello, world!");
}
