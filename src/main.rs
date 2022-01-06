use std::time::{SystemTime, UNIX_EPOCH};

use id_5_smallest_multiple;

fn main() {
    let t0 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    let answer = id_5_smallest_multiple::smallest_multiple(10);
    let t1=SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    println!("{}",answer);
    println!("Execution time: {} milliseconds", t1.as_secs_f64() * 1000.0 - t0.as_secs_f64() * 1000.0)
}
