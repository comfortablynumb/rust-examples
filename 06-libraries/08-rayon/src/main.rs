use rayon::prelude::*;
use std::time::Instant;

fn main() {
    println!("Rayon Data Parallelism Examples\n");

    // Parallel iteration
    parallel_iteration();

    // Parallel map
    parallel_map();

    // Parallel filter
    parallel_filter();

    // Parallel sorting
    parallel_sort();

    // Parallel reduce
    parallel_reduce();

    // Performance comparison
    performance_comparison();

    // Custom thread pool
    custom_thread_pool();
}

fn parallel_iteration() {
    println!("=== Parallel Iteration ===");

    let numbers: Vec<i32> = (0..10).collect();

    // Sequential
    println!("Sequential:");
    numbers.iter().for_each(|&n| {
        println!("  Processing {}", n);
    });

    // Parallel
    println!("\nParallel (order may vary):");
    numbers.par_iter().for_each(|&n| {
        println!("  Processing {}", n);
    });

    println!();
}

fn parallel_map() {
    println!("=== Parallel Map ===");

    let numbers: Vec<i32> = (1..=10).collect();

    // Square each number in parallel
    let squares: Vec<i32> = numbers.par_iter().map(|&n| n * n).collect();

    println!("Original: {:?}", numbers);
    println!("Squared:  {:?}", squares);
    println!();
}

fn parallel_filter() {
    println!("=== Parallel Filter ===");

    let numbers: Vec<i32> = (1..=20).collect();

    // Filter even numbers in parallel
    let evens: Vec<i32> = numbers
        .par_iter()
        .filter(|&&n| n % 2 == 0)
        .copied()
        .collect();

    println!("Numbers: {:?}", numbers);
    println!("Evens:   {:?}", evens);
    println!();
}

fn parallel_sort() {
    println!("=== Parallel Sort ===");

    let mut numbers: Vec<i32> = vec![64, 34, 25, 12, 22, 11, 90, 88, 45, 50, 23, 36, 18, 9, 80];

    println!("Unsorted: {:?}", numbers);

    // Parallel sort
    numbers.par_sort();

    println!("Sorted:   {:?}", numbers);
    println!();
}

fn parallel_reduce() {
    println!("=== Parallel Reduce ===");

    let numbers: Vec<i32> = (1..=100).collect();

    // Sum in parallel
    let sum: i32 = numbers.par_iter().sum();

    println!("Sum of 1..=100: {}", sum);

    // Product (careful with overflow!)
    let product: i64 = (1..=10).into_par_iter().map(|n| n as i64).product();

    println!("Product of 1..=10: {}", product);

    // Custom reduce
    let max = numbers
        .par_iter()
        .reduce(|| &0, |a, b| if a > b { a } else { b });

    println!("Max: {}", max);
    println!();
}

fn performance_comparison() {
    println!("=== Performance Comparison ===");

    // Large dataset
    let numbers: Vec<i32> = (0..1_000_000).collect();

    // Sequential
    let start = Instant::now();
    let seq_sum: i64 = numbers.iter().map(|&n| n as i64 * n as i64).sum();
    let seq_time = start.elapsed();

    // Parallel
    let start = Instant::now();
    let par_sum: i64 = numbers.par_iter().map(|&n| n as i64 * n as i64).sum();
    let par_time = start.elapsed();

    println!("Dataset size: {} elements", numbers.len());
    println!("Sequential sum: {} (took {:?})", seq_sum, seq_time);
    println!("Parallel sum:   {} (took {:?})", par_sum, par_time);
    println!(
        "Speedup: {:.2}x",
        seq_time.as_secs_f64() / par_time.as_secs_f64()
    );
    println!();
}

fn custom_thread_pool() {
    println!("=== Custom Thread Pool ===");

    // Create custom thread pool with 4 threads
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.install(|| {
        let sum: i32 = (1..=100).into_par_iter().sum();

        println!("Sum computed in custom pool (4 threads): {}", sum);
    });

    println!();
}
