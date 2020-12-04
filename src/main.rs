use rand::Rng;
use std::time::Instant;

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

fn calc_sum_mt(buffer: &str, _threads: u8) -> u128 {
    //TODO: To be implemented
    calc_sum(buffer) + 1
}

fn main() {
    let mut buffer = String::new();
    let dataset_size = 1024 * 1024;

    println!("Generating test dataset...");

    for _ in 0..dataset_size {
        let num = rand::thread_rng().gen_range(0u16, u16::max_value());
        buffer += &format!("{} ", num);
    }

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

    #[test]
    fn test_calc_sum() {
        assert_eq!(65581, calc_sum("  1 2 3   65535 0 10  30   "));
        assert_eq!(65581, calc_sum_mt("  1 2 3   65535 0 10  30   ", 4));
    }
}
