use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::split;

fn imitate_cpu_load() {
    for _ in 0..10 {
        rand::thread_rng().gen_range(0u16, u16::max_value());
    }
}

pub fn calc_sum(buffer: &str) -> u128 {
    buffer
        .split_whitespace()
        .fold(0u128, |acc, str| match str.parse::<u16>() {
            Ok(val) => {
                imitate_cpu_load();
                acc + val as u128
            }
            Err(_) => acc,
        })
}

pub fn calc_sum_mt(buffer: &Arc<String>, threads: usize) -> u128 {
    let pieces = split::split(buffer, threads);
    let result = Arc::new(Mutex::new(0u128));

    let mut handles = Vec::<JoinHandle<_>>::new();

    for piece in pieces {
        let results_clone = Arc::clone(&result);

        handles.push(thread::spawn(move || {
            let res = calc_sum(piece.slice());
            let mut shared_res = results_clone.lock().unwrap();

            *shared_res += res
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let res = result.lock().unwrap();

    *res
}

#[cfg(test)]
mod tests {
    use crate::calc::{calc_sum, calc_sum_mt};
    use std::sync::Arc;

    #[test]
    fn test_calc() {
        let test_buf = "1 2 3 65535 0 10 30 ";

        assert_eq!(65581, calc_sum(test_buf));
        assert_eq!(65581, calc_sum_mt(&Arc::new(String::from(test_buf)), 4));
    }
}
