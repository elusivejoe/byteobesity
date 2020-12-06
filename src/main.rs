use rand::Rng;
use std::sync::Arc;
use std::time::Instant;

mod calc;
use calc::{calc_sum, calc_sum_mt};

mod split;

fn generate_dataset(size_bytes: usize) -> Arc<String> {
    let mut buffer = String::new();

    while buffer.len() < size_bytes {
        let num = rand::thread_rng().gen_range(0u16, u16::max_value());
        buffer += &format!("{} ", num);
    }

    Arc::new(buffer)
}

fn main() {
    println!("Generating test dataset...");

    let dataset_size = 1024 * 1024 * 1024 * 2;
    let buffer = generate_dataset(dataset_size);

    {
        println!("\nCalculating in single thread...");

        let start_time = Instant::now();

        println!("Result: {}", calc_sum(&buffer));

        println!(
            "Time spent: {} sec",
            Instant::now().duration_since(start_time).as_secs()
        );
    }

    {
        let threads = 8;

        println!("\nCalculating in {} threads...", threads);

        let start_time = Instant::now();

        println!("Result: {}", calc_sum_mt(&buffer, threads));

        println!(
            "Time spent: {} sec",
            Instant::now().duration_since(start_time).as_secs()
        );
    }
}
