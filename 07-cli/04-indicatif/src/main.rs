//! Indicatif Progress Bars and Spinners Example
//!
//! This example demonstrates comprehensive usage of indicatif for
//! creating beautiful progress indicators in CLI applications:
//! - Single progress bars with various styles
//! - Spinners for indeterminate operations
//! - Multi-progress for parallel tasks
//! - Integration with real work (file processing simulation)
//! - Custom templates and styling

use indicatif::{
    DecimalBytes, HumanBytes, HumanDuration, MultiProgress, ProgressBar, ProgressDrawTarget,
    ProgressFinish, ProgressState, ProgressStyle,
};
use rand::Rng;
use std::{
    fmt::Write as _,
    io::{self, Write},
    thread,
    time::Duration,
};

fn main() -> io::Result<()> {
    loop {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen
        io::stdout().flush()?;

        println!("╔═══════════════════════════════════════════════════════════╗");
        println!("║         Indicatif Progress Bars & Spinners Demo          ║");
        println!("╚═══════════════════════════════════════════════════════════╝");
        println!();
        println!("Choose a demo:");
        println!();
        println!("  1. Basic Progress Bar");
        println!("  2. Styled Progress Bars");
        println!("  3. Spinners");
        println!("  4. Multi-Progress (Parallel Tasks)");
        println!("  5. File Processing Simulation");
        println!("  6. Download Simulation");
        println!("  7. Complex Multi-Stage Operation");
        println!("  8. Exit");
        println!();
        print!("Enter your choice (1-8): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "1" => basic_progress_bar(),
            "2" => styled_progress_bars(),
            "3" => spinners_demo(),
            "4" => multi_progress_demo(),
            "5" => file_processing_demo(),
            "6" => download_simulation(),
            "7" => complex_multi_stage(),
            "8" => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("\nInvalid choice, please try again."),
        }

        if input.trim() != "8" {
            println!("\nPress Enter to continue...");
            let mut pause = String::new();
            io::stdin().read_line(&mut pause)?;
        }
    }

    Ok(())
}

/// Demo 1: Basic progress bar
fn basic_progress_bar() {
    println!("\n═══ Basic Progress Bar ═══\n");

    // Create a simple progress bar for 100 items
    let pb = ProgressBar::new(100);

    for i in 0..100 {
        thread::sleep(Duration::from_millis(50));
        pb.inc(1);

        // Optionally set a message
        if i % 10 == 0 {
            pb.set_message(format!("Processing item {}", i));
        }
    }

    pb.finish_with_message("Complete!");
}

/// Demo 2: Different styled progress bars
fn styled_progress_bars() {
    println!("\n═══ Styled Progress Bars ═══\n");

    // Style 1: Default style
    println!("1. Default Style:");
    let pb1 = ProgressBar::new(100);
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(20));
        pb1.inc(1);
    }
    pb1.finish_and_clear();

    // Style 2: With template
    println!("\n2. Custom Template:");
    let pb2 = ProgressBar::new(100);
    pb2.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );

    for i in 0..100 {
        thread::sleep(Duration::from_millis(20));
        pb2.set_message(format!("Item #{}", i + 1));
        pb2.inc(1);
    }
    pb2.finish_with_message("Done!");

    // Style 3: Percentage and eta
    println!("\n3. With Percentage and ETA:");
    let pb3 = ProgressBar::new(100);
    pb3.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) ETA: {eta}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );

    for _ in 0..100 {
        thread::sleep(Duration::from_millis(30));
        pb3.inc(1);
    }
    pb3.finish_with_message("Complete!");

    // Style 4: Bytes processing
    println!("\n4. Bytes Processing:");
    let pb4 = ProgressBar::new(1024 * 1024 * 10); // 10 MB
    pb4.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40}] {bytes}/{total_bytes} ({bytes_per_sec}) {msg}")
            .unwrap()
            .progress_chars("██░"),
    );

    let mut processed = 0;
    while processed < 1024 * 1024 * 10 {
        let chunk = rand::thread_rng().gen_range(8192..65536);
        processed += chunk;
        pb4.set_position(processed.min(1024 * 1024 * 10));
        thread::sleep(Duration::from_millis(5));
    }
    pb4.finish_with_message("Processing complete!");

    // Style 5: Custom function
    println!("\n5. With Custom Function:");
    let pb5 = ProgressBar::new(100);
    pb5.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40}] {pos}/{len} {msg}")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            })
            .progress_chars("█▓▒░  "),
    );

    for i in 0..100 {
        thread::sleep(Duration::from_millis(25));
        pb5.set_message(format!("Phase {}", i / 20 + 1));
        pb5.inc(1);
    }
    pb5.finish_with_message("All phases complete!");
}

/// Demo 3: Spinners for indeterminate operations
fn spinners_demo() {
    println!("\n═══ Spinners Demo ═══\n");

    // Spinner 1: Default spinner
    println!("1. Default Spinner:");
    let pb1 = ProgressBar::new_spinner();
    pb1.enable_steady_tick(Duration::from_millis(100));
    pb1.set_message("Loading...");

    for _ in 0..50 {
        thread::sleep(Duration::from_millis(50));
        pb1.tick();
    }
    pb1.finish_with_message("Loaded!");

    // Spinner 2: Custom spinner frames
    println!("\n2. Custom Spinner (dots):");
    let pb2 = ProgressBar::new_spinner();
    pb2.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    pb2.enable_steady_tick(Duration::from_millis(100));
    pb2.set_message("Processing data...");

    for _ in 0..40 {
        thread::sleep(Duration::from_millis(75));
    }
    pb2.finish_with_message("✓ Data processed");

    // Spinner 3: Box spinner
    println!("\n3. Box Spinner:");
    let pb3 = ProgressBar::new_spinner();
    pb3.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["◐", "◓", "◑", "◒"])
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap(),
    );
    pb3.enable_steady_tick(Duration::from_millis(150));
    pb3.set_message("Connecting to server...");

    for i in 0..30 {
        thread::sleep(Duration::from_millis(100));
        if i % 10 == 0 {
            pb3.set_message(format!("Retrying connection... (attempt {})", i / 10 + 1));
        }
    }
    pb3.finish_with_message("✓ Connected successfully");

    // Spinner 4: Arrow spinner
    println!("\n4. Arrow Spinner:");
    let pb4 = ProgressBar::new_spinner();
    pb4.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb4.enable_steady_tick(Duration::from_millis(100));
    pb4.set_message("Scanning directories...");

    for _ in 0..50 {
        thread::sleep(Duration::from_millis(60));
    }
    pb4.finish_with_message("✓ Scan complete");
}

/// Demo 4: Multiple parallel progress bars
fn multi_progress_demo() {
    println!("\n═══ Multi-Progress Demo ═══\n");

    let m = MultiProgress::new();

    // Create multiple progress bars
    let pb1 = m.add(ProgressBar::new(128));
    pb1.set_style(
        ProgressStyle::default_bar()
            .template("[Task 1] {bar:40.cyan/blue} {pos:>3}/{len:3}")
            .unwrap()
            .progress_chars("█▓▒░  "),
    );

    let pb2 = m.add(ProgressBar::new(128));
    pb2.set_style(
        ProgressStyle::default_bar()
            .template("[Task 2] {bar:40.green/yellow} {pos:>3}/{len:3}")
            .unwrap()
            .progress_chars("█▓▒░  "),
    );

    let pb3 = m.add(ProgressBar::new(128));
    pb3.set_style(
        ProgressStyle::default_bar()
            .template("[Task 3] {bar:40.magenta/red} {pos:>3}/{len:3}")
            .unwrap()
            .progress_chars("█▓▒░  "),
    );

    // Simulate parallel work with threads
    let handles: Vec<_> = vec![(pb1, 30, 128), (pb2, 45, 128), (pb3, 60, 128)]
        .into_iter()
        .map(|(pb, delay, total)| {
            thread::spawn(move || {
                for _ in 0..total {
                    thread::sleep(Duration::from_millis(delay));
                    pb.inc(1);
                }
                pb.finish();
            })
        })
        .collect();

    // Wait for all tasks to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("\n✓ All tasks completed!");
}

/// Demo 5: File processing simulation
fn file_processing_demo() {
    println!("\n═══ File Processing Simulation ═══\n");

    // Simulate processing multiple files
    let files = vec![
        ("document.pdf", 1024 * 512),
        ("image.jpg", 1024 * 256),
        ("video.mp4", 1024 * 1024 * 5),
        ("archive.zip", 1024 * 768),
        ("data.csv", 1024 * 128),
    ];

    let m = MultiProgress::new();

    let main_pb = m.add(ProgressBar::new(files.len() as u64));
    main_pb.set_style(
        ProgressStyle::default_bar()
            .template("Overall: {bar:40.cyan/blue} {pos}/{len} files")
            .unwrap()
            .progress_chars("█▓░"),
    );

    for (filename, size) in files {
        let pb = m.add(ProgressBar::new(size));
        pb.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "{{spinner:.green}} {{msg:<20}} [{{bar:30}}] {{bytes}}/{{total_bytes}} ({{bytes_per_sec}})"
                ))
                .unwrap()
                .progress_chars("█▓▒░  "),
        );
        pb.set_message(filename.to_string());

        // Simulate processing
        let mut processed = 0;
        while processed < size {
            let chunk = rand::thread_rng()
                .gen_range(4096..16384)
                .min(size - processed);
            processed += chunk;
            pb.set_position(processed);
            thread::sleep(Duration::from_millis(5));
        }

        pb.finish_with_message(format!("✓ {}", filename));
        main_pb.inc(1);
    }

    main_pb.finish_with_message("All files processed!");
}

/// Demo 6: Download simulation
fn download_simulation() {
    println!("\n═══ Download Simulation ═══\n");

    let downloads = vec![
        ("package-1.tar.gz", 1024 * 1024 * 3),
        ("package-2.tar.gz", 1024 * 1024 * 7),
        ("package-3.tar.gz", 1024 * 1024 * 2),
    ];

    let m = MultiProgress::new();

    let overall_size: u64 = downloads.iter().map(|(_, size)| size).sum();
    let overall_pb = m.add(ProgressBar::new(overall_size));
    overall_pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "Total: [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}) ETA: {eta}",
            )
            .unwrap()
            .progress_chars("█▓▒░  "),
    );

    let mut handles = vec![];

    for (filename, size) in downloads {
        let pb = m.insert_before(&overall_pb, ProgressBar::new(size));
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{msg:<25} [{bar:30.green}] {bytes:>10}/{total_bytes:>10} {bytes_per_sec:>15}",
                )
                .unwrap()
                .progress_chars("=>-"),
        );
        pb.set_message(filename.to_string());

        let overall_pb_clone = overall_pb.clone();

        let handle = thread::spawn(move || {
            let mut downloaded = 0;
            while downloaded < size {
                let chunk = rand::thread_rng()
                    .gen_range(8192..32768)
                    .min(size - downloaded);
                downloaded += chunk;
                pb.inc(chunk);
                overall_pb_clone.inc(chunk);
                thread::sleep(Duration::from_millis(10));
            }
            pb.finish_with_message(format!("✓ {}", filename));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    overall_pb.finish_with_message("All downloads complete!");
}

/// Demo 7: Complex multi-stage operation
fn complex_multi_stage() {
    println!("\n═══ Complex Multi-Stage Operation ═══\n");

    let m = MultiProgress::new();

    // Stage 1: Initialization
    let init_pb = m.add(ProgressBar::new_spinner());
    init_pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    init_pb.enable_steady_tick(Duration::from_millis(100));
    init_pb.set_message("Initializing...");
    thread::sleep(Duration::from_secs(2));
    init_pb.finish_with_message("✓ Initialization complete");

    // Stage 2: Data collection
    let collect_pb = m.add(ProgressBar::new(50));
    collect_pb.set_style(
        ProgressStyle::default_bar()
            .template("Collecting: {bar:40.green} {pos}/{len} items")
            .unwrap()
            .progress_chars("█▓░"),
    );

    for _ in 0..50 {
        thread::sleep(Duration::from_millis(40));
        collect_pb.inc(1);
    }
    collect_pb.finish_with_message("✓ Collection complete");

    // Stage 3: Parallel processing
    let process_pb = m.add(ProgressBar::new(100));
    process_pb.set_style(
        ProgressStyle::default_bar()
            .template("Processing: {bar:40.yellow} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("▰▱"),
    );

    for i in 0..100 {
        thread::sleep(Duration::from_millis(30));
        if i % 25 == 0 && i > 0 {
            process_pb.set_message(format!("Phase {}/4", i / 25));
        }
        process_pb.inc(1);
    }
    process_pb.finish_with_message("✓ Processing complete");

    // Stage 4: Validation
    let validate_pb = m.add(ProgressBar::new_spinner());
    validate_pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.magenta} {msg}")
            .unwrap()
            .tick_strings(&["◐", "◓", "◑", "◒"]),
    );
    validate_pb.enable_steady_tick(Duration::from_millis(150));
    validate_pb.set_message("Validating results...");
    thread::sleep(Duration::from_secs(3));
    validate_pb.finish_with_message("✓ Validation complete");

    // Stage 5: Finalization
    let final_pb = m.add(ProgressBar::new(20));
    final_pb.set_style(
        ProgressStyle::default_bar()
            .template("Finalizing: {bar:40.blue} {pos}/{len}")
            .unwrap()
            .progress_chars("=>-"),
    );

    for _ in 0..20 {
        thread::sleep(Duration::from_millis(100));
        final_pb.inc(1);
    }
    final_pb.finish_with_message("✓ Finalization complete");

    println!("\n🎉 All stages completed successfully!");
}
