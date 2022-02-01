use rand::Rng;
use std::{thread, time};

fn main() {
    let mut rng = rand::thread_rng();
    let millis = time::Duration::from_millis(500);
    for _ in 1..11 {
        let rn: u8 = rng.gen();
        let result: &'static str = if rn % 2 == 0 {"Pass"} else {"Fail"};
        println!("Test #{} {}", rn, result);
        thread::sleep(millis);
    }
}