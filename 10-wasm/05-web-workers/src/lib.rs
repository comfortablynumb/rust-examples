use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

/// Message types for worker communication
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerMessage {
    pub task: String,
    pub data: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerResult {
    pub task: String,
    pub result: f64,
    pub time_ms: f64,
}

/// Calculate prime numbers up to n (CPU-intensive task)
#[wasm_bindgen]
pub fn calculate_primes(n: u32) -> Vec<u32> {
    let start = js_sys::Date::now();

    let mut primes = Vec::new();
    for num in 2..=n {
        if is_prime(num) {
            primes.push(num);
        }
    }

    let elapsed = js_sys::Date::now() - start;
    console::log_1(&format!("Found {} primes in {}ms", primes.len(), elapsed).into());

    primes
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Calculate Fibonacci number (recursive, inefficient on purpose for demonstration)
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// Monte Carlo Pi estimation
#[wasm_bindgen]
pub fn estimate_pi(iterations: u32) -> f64 {
    let start = js_sys::Date::now();

    let mut inside_circle = 0;

    for _ in 0..iterations {
        let x = js_sys::Math::random();
        let y = js_sys::Math::random();

        if x * x + y * y <= 1.0 {
            inside_circle += 1;
        }
    }

    let pi_estimate = 4.0 * (inside_circle as f64) / (iterations as f64);

    let elapsed = js_sys::Date::now() - start;
    console::log_1(&format!("Pi estimated as {} in {}ms", pi_estimate, elapsed).into());

    pi_estimate
}

/// Matrix multiplication
#[wasm_bindgen]
pub fn matrix_multiply(size: usize) -> f64 {
    let start = js_sys::Date::now();

    // Create two matrices filled with random values
    let mut a = vec![vec![0.0; size]; size];
    let mut b = vec![vec![0.0; size]; size];
    let mut c = vec![vec![0.0; size]; size];

    for i in 0..size {
        for j in 0..size {
            a[i][j] = js_sys::Math::random();
            b[i][j] = js_sys::Math::random();
        }
    }

    // Multiply
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    let elapsed = js_sys::Date::now() - start;
    console::log_1(&format!("Matrix multiply ({}x{}) in {}ms", size, size, elapsed).into());

    c[0][0] // Return something to prevent optimization
}

/// Sort a large array
#[wasm_bindgen]
pub fn sort_array(mut data: Vec<f64>) -> Vec<f64> {
    let start = js_sys::Date::now();

    data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let elapsed = js_sys::Date::now() - start;
    console::log_1(&format!("Sorted {} elements in {}ms", data.len(), elapsed).into());

    data
}

/// Calculate statistics on a dataset
#[wasm_bindgen]
pub fn calculate_statistics(data: Vec<f64>) -> JsValue {
    let start = js_sys::Date::now();

    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;

    let variance = data.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / n;

    let std_dev = variance.sqrt();

    let mut sorted = data.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median = if sorted.len() % 2 == 0 {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
    } else {
        sorted[sorted.len() / 2]
    };

    let elapsed = js_sys::Date::now() - start;

    let result = serde_wasm_bindgen::to_value(&serde_json::json!({
        "mean": mean,
        "median": median,
        "std_dev": std_dev,
        "min": sorted.first().unwrap(),
        "max": sorted.last().unwrap(),
        "time_ms": elapsed,
    })).unwrap();

    result
}

/// Simulate heavy computation
#[wasm_bindgen]
pub fn heavy_computation(iterations: u32) -> u32 {
    let start = js_sys::Date::now();

    let mut result = 0u32;
    for i in 0..iterations {
        result = result.wrapping_add(i);
        // Simulate work
        for j in 0..1000 {
            result = result.wrapping_mul(j.wrapping_add(1));
        }
    }

    let elapsed = js_sys::Date::now() - start;
    console::log_1(&format!("Heavy computation completed in {}ms", elapsed).into());

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(7));
        assert!(!is_prime(8));
        assert!(is_prime(13));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }
}
