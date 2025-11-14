use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Filesystem Watching Example\n");

    // Create test directory
    let watch_path = "./watch_test";
    std::fs::create_dir_all(watch_path)?;
    println!("Created watch directory: {}", watch_path);

    // Create channel for receiving events
    let (tx, rx) = channel();

    // Create watcher
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            tx.send(res).unwrap();
        },
        Config::default(),
    )?;

    // Start watching
    watcher.watch(Path::new(watch_path), RecursiveMode::Recursive)?;
    println!("Watching {} for changes...\n", watch_path);

    // Simulate some filesystem operations
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_millis(500));
        println!("Simulating filesystem changes...\n");

        // Create file
        std::fs::write("./watch_test/test1.txt", "Hello, World!").ok();
        std::thread::sleep(Duration::from_millis(200));

        // Modify file
        std::fs::write("./watch_test/test1.txt", "Modified content!").ok();
        std::thread::sleep(Duration::from_millis(200));

        // Create another file
        std::fs::write("./watch_test/test2.txt", "Another file").ok();
        std::thread::sleep(Duration::from_millis(200));

        // Delete file
        std::fs::remove_file("./watch_test/test1.txt").ok();
        std::thread::sleep(Duration::from_millis(200));

        // Create directory
        std::fs::create_dir("./watch_test/subdir").ok();
        std::thread::sleep(Duration::from_millis(200));
    });

    // Process events
    let mut event_count = 0;
    let max_events = 20; // Limit for demo

    for res in rx {
        match res {
            Ok(event) => {
                print_event(&event);
                event_count += 1;

                if event_count >= max_events {
                    println!("\nReached maximum events, stopping...");
                    break;
                }
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }

    // Cleanup
    std::fs::remove_dir_all(watch_path)?;
    println!("\nCleaned up test directory");

    Ok(())
}

fn print_event(event: &Event) {
    let kind_str = match &event.kind {
        EventKind::Create(_) => "CREATE",
        EventKind::Modify(_) => "MODIFY",
        EventKind::Remove(_) => "REMOVE",
        EventKind::Access(_) => "ACCESS",
        _ => "OTHER",
    };

    for path in &event.paths {
        println!("[{}] {}", kind_str, path.display());
    }
}
