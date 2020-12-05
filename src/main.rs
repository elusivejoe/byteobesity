use crate::split::Piece;
use rand::Rng;
use std::sync::Arc;
use std::time::Instant;

mod split;

fn imitate_cpu_load() {
    for _ in 0..10 {
        rand::thread_rng().gen_range(0u16, u16::max_value());
    }
}

fn calc_sum(buffer: &str) -> u128 {
    buffer
        .split(" ")
        .fold(0u128, |acc, str| match str.parse::<u16>() {
            Ok(val) => {
                imitate_cpu_load();
                acc + val as u128
            }
            Err(_) => acc,
        })
}

fn calc_sum_mt(buffer: &Arc<String>, _threads: u8) -> u128 {
    let piece = Piece::new(buffer, 0, buffer.len());

    calc_sum(piece.slice()) + 1
}

fn generate_dataset(size: i32) -> Arc<String> {
    let mut buffer = String::new();

    for _ in 0..size {
        let num = rand::thread_rng().gen_range(0u16, u16::max_value());
        buffer += &format!("{} ", num);
    }

    Arc::new(buffer)
}

fn main() {
    let dataset_size = 1024 * 1024;

    println!("Generating test dataset...");

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
        let threads = 4;

        println!("\nCalculating in {} threads...", threads);
        let start_time = Instant::now();

        println!("Result: {}", calc_sum_mt(&buffer, threads));

        println!(
            "Time spent: {} sec",
            Instant::now().duration_since(start_time).as_secs()
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{calc_sum, calc_sum_mt};
    use std::sync::Arc;

    #[test]
    fn test_calc_sum() {
        let test_buf = "  1 2 3   65535 0 10  30   ";

        assert_eq!(65581, calc_sum(test_buf));
        assert_eq!(65581, calc_sum_mt(&Arc::new(String::from(test_buf)), 4));
    }
}
