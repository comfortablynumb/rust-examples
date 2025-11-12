# Indicatif Progress Bars and Spinners

Comprehensive examples demonstrating indicatif for creating beautiful progress indicators in CLI applications, from simple progress bars to complex multi-stage operations.

## Features Demonstrated

### 1. **Basic Progress Bars**
- Simple progress bar with item counter
- Progress messages
- Automatic finish handling

### 2. **Styled Progress Bars**
- Custom templates and formatting
- Color-coded progress indicators
- Different progress characters (=>-, █▓░, etc.)
- Percentage and ETA display
- Byte processing with human-readable formats
- Custom formatter functions

### 3. **Spinners**
- Indeterminate progress indicators
- Custom spinner frames
- Different spinner styles:
  - Dots (⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏)
  - Box rotation (◐◓◑◒)
  - Arrows (←↖↑↗→↘↓↙)
- Elapsed time tracking

### 4. **Multi-Progress**
- Multiple simultaneous progress bars
- Parallel task visualization
- Thread-safe progress tracking
- Coordinated progress display

### 5. **Real-World Simulations**
- File processing with byte counts
- Download progress with transfer speeds
- Complex multi-stage operations
- Overall progress with individual task progress

## Usage

```bash
# Run the demo application
cargo run

# Select from the menu:
# 1. Basic Progress Bar
# 2. Styled Progress Bars
# 3. Spinners
# 4. Multi-Progress (Parallel Tasks)
# 5. File Processing Simulation
# 6. Download Simulation
# 7. Complex Multi-Stage Operation
# 8. Exit
```

## Code Examples

### Basic Progress Bar

```rust
let pb = ProgressBar::new(100);

for i in 0..100 {
    thread::sleep(Duration::from_millis(50));
    pb.inc(1);

    if i % 10 == 0 {
        pb.set_message(format!("Processing item {}", i));
    }
}

pb.finish_with_message("Complete!");
```

### Custom Styled Bar

```rust
let pb = ProgressBar::new(100);
pb.set_style(
    ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("=>-"),
);
```

### Spinner

```rust
let spinner = ProgressBar::new_spinner();
spinner.set_style(
    ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner:.blue} {msg}")
        .unwrap(),
);
spinner.enable_steady_tick(Duration::from_millis(100));
spinner.set_message("Loading...");

// Do work...

spinner.finish_with_message("✓ Done");
```

### Multi-Progress

```rust
let m = MultiProgress::new();

let pb1 = m.add(ProgressBar::new(128));
pb1.set_style(/* ... */);

let pb2 = m.add(ProgressBar::new(128));
pb2.set_style(/* ... */);

// Spawn threads for parallel work
let handles: Vec<_> = vec![(pb1, 30), (pb2, 45)]
    .into_iter()
    .map(|(pb, delay)| {
        thread::spawn(move || {
            for _ in 0..128 {
                thread::sleep(Duration::from_millis(delay));
                pb.inc(1);
            }
            pb.finish();
        })
    })
    .collect();

// Wait for completion
for handle in handles {
    handle.join().unwrap();
}
```

## Template Variables

Available variables for custom templates:

### Progress Bars
- `{bar}` - The progress bar
- `{pos}` - Current position
- `{len}` - Total length
- `{percent}` - Percentage (0-100)
- `{msg}` - Custom message
- `{wide_msg}` - Truncated message
- `{elapsed}` - Elapsed time (e.g., "2m")
- `{elapsed_precise}` - Precise elapsed time (e.g., "1:23.45")
- `{eta}` - Estimated time remaining
- `{eta_precise}` - Precise ETA
- `{bytes}` - Current bytes (human-readable)
- `{total_bytes}` - Total bytes (human-readable)
- `{bytes_per_sec}` - Transfer rate
- `{spinner}` - Spinner animation

### Spinners
- `{spinner}` - Spinner animation
- `{msg}` - Custom message
- `{elapsed}` - Elapsed time
- `{elapsed_precise}` - Precise elapsed time

## Progress Characters

Common progress character sets:

```rust
// Classic
.progress_chars("=>-")

// Blocks
.progress_chars("█▓▒░  ")

// Unicode blocks
.progress_chars("█▉▊▋▌▍▎▏  ")

// Simple
.progress_chars("##-")

// Dots
.progress_chars("●◐○")
```

## Color Formatting

Template colors use the format `{element:width.color/background}`:

```rust
"{bar:40.cyan/blue}"     // Cyan bar, blue background, 40 chars wide
"{spinner:.green}"       // Green spinner
"{msg:.yellow}"          // Yellow message
```

Available colors: black, red, green, yellow, blue, magenta, cyan, white

## Best Practices

### 1. Always Finish Progress Bars

```rust
// Finish with message
pb.finish_with_message("Done!");

// Finish and clear
pb.finish_and_clear();

// Abandon if error
pb.abandon_with_message("Failed!");
```

### 2. Use Appropriate Finish Behavior

```rust
// Set finish behavior
pb.set_finish(ProgressFinish::AndLeave);    // Default
pb.set_finish(ProgressFinish::AndClear);    // Clear on finish
pb.set_finish(ProgressFinish::Abandon);     // Leave incomplete
```

### 3. Thread Safety

```rust
// Clone for threads
let pb_clone = pb.clone();
thread::spawn(move || {
    pb_clone.inc(1);
});
```

### 4. Byte Formatting

```rust
// Use built-in byte formatters
.template("{bytes}/{total_bytes} ({bytes_per_sec})")

// For specific values
use indicatif::HumanBytes;
println!("{}", HumanBytes(12345678));  // "12.35 MB"
```

### 5. Steady Tick for Spinners

```rust
// Enable auto-tick for smooth animation
spinner.enable_steady_tick(Duration::from_millis(100));

// No need to manually call .tick()
```

## Real-World Integration

### File Processing

```rust
fn process_file(path: &Path, pb: &ProgressBar) -> io::Result<()> {
    let file = File::open(path)?;
    let size = file.metadata()?.len();

    pb.set_length(size);

    let mut reader = BufReader::new(file);
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        // Process data...

        pb.inc(bytes_read as u64);
    }

    pb.finish();
    Ok(())
}
```

### Download with Progress

```rust
async fn download_file(url: &str, pb: &ProgressBar) -> Result<()> {
    let response = reqwest::get(url).await?;
    let total_size = response.content_length().unwrap_or(0);

    pb.set_length(total_size);

    let mut stream = response.bytes_stream();
    let mut file = File::create("output.bin")?;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
        pb.inc(chunk.len() as u64);
    }

    pb.finish();
    Ok(())
}
```

### Parallel Processing

```rust
fn process_items_parallel(items: Vec<Item>) {
    let m = MultiProgress::new();
    let overall = m.add(ProgressBar::new(items.len() as u64));

    let handles: Vec<_> = items
        .into_iter()
        .map(|item| {
            let pb = m.add(ProgressBar::new(item.work_units()));
            let overall = overall.clone();

            thread::spawn(move || {
                // Process item...
                pb.finish();
                overall.inc(1);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    overall.finish();
}
```

## Performance Tips

1. **Batch Updates** - Update progress in chunks, not per byte
2. **Reasonable Draw Target** - Default is usually fine
3. **Finish When Done** - Always call finish to clean up
4. **Minimize Template Complexity** - Keep templates simple for better performance

## Common Patterns

### Success/Failure Messages

```rust
if result.is_ok() {
    pb.finish_with_message("✓ Success");
} else {
    pb.abandon_with_message("✗ Failed");
}
```

### Nested Progress

```rust
let outer = ProgressBar::new(files.len() as u64);
for file in files {
    let inner = ProgressBar::new(file.size());
    process_file(file, &inner);
    inner.finish();
    outer.inc(1);
}
outer.finish();
```

### Hidden Progress

```rust
// Hide in non-interactive environments
let pb = if is_tty() {
    ProgressBar::new(100)
} else {
    ProgressBar::hidden()
};
```

## Dependencies

- **indicatif** - Progress bar and spinner library
- **rand** - Random number generation for demos

## Learning Resources

- [Indicatif Documentation](https://docs.rs/indicatif/)
- [Indicatif GitHub](https://github.com/console-rs/indicatif)
- [Examples](https://github.com/console-rs/indicatif/tree/main/examples)

## Production Considerations

1. **Terminal Detection** - Disable progress in non-TTY environments
2. **Logging Integration** - Use `ProgressBar::println` for log messages
3. **Error Handling** - Always abandon progress on errors
4. **Resource Cleanup** - Finish or abandon all progress bars
5. **User Preference** - Provide flags to disable progress output
