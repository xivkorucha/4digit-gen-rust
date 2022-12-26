use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut rng = thread_rng();
    let mut numbers: Vec<u32> = (0000..10000).collect();
    numbers.shuffle(&mut rng);

    for number in numbers {
        println!("{:04}", number);
    }

    let elapsed = start.elapsed().as_secs_f64();
    println!("Elapsed time: {:.2} seconds", elapsed);
}
