// Enums in Rust
//
// Enumerations (enums) allow you to define a type by enumerating its possible variants.
// Unlike enums in other languages, Rust enums can hold data and are algebraic data types.

fn main() {
    println!("=== Rust Enums Examples ===\n");

    // Example 1: Basic enums
    println!("1. Basic Enums:");
    basic_enums();
    println!();

    // Example 2: Enums with data
    println!("2. Enums with Data:");
    enums_with_data();
    println!();

    // Example 3: Methods on enums
    println!("3. Methods on Enums:");
    enum_methods();
    println!();

    // Example 4: Option enum
    println!("4. Option Enum:");
    option_examples();
    println!();

    // Example 5: Result enum
    println!("5. Result Enum:");
    result_examples();
    println!();

    // Example 6: Complex enum variants
    println!("6. Complex Enum Variants:");
    complex_variants();
    println!();

    // Example 7: Enums in data structures
    println!("7. Enums in Data Structures:");
    enums_in_structures();
    println!();
}

// Example 1: Basic enums without data
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn basic_enums() {
    let direction = Direction::North;
    println!("  Going {:?}", direction);

    // Match on enum
    match direction {
        Direction::North => println!("  Heading north!"),
        Direction::South => println!("  Heading south!"),
        Direction::East => println!("  Heading east!"),
        Direction::West => println!("  Heading west!"),
    }

    // Using in functions
    fn move_player(dir: Direction) -> String {
        match dir {
            Direction::North => "Moved up".to_string(),
            Direction::South => "Moved down".to_string(),
            Direction::East => "Moved right".to_string(),
            Direction::West => "Moved left".to_string(),
        }
    }

    println!("  {}", move_player(Direction::East));
}

// Example 2: Enums with different data types
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },   // Named fields like a struct
    Write(String),              // Single String
    ChangeColor(i32, i32, i32), // Three i32 values
}

fn enums_with_data() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("  Home IP: {:?}", home);
    println!("  Loopback IP: {:?}", loopback);

    // Extracting data from enums
    match home {
        IpAddr::V4(a, b, c, d) => {
            println!("  IPv4 address: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(addr) => {
            println!("  IPv6 address: {}", addr);
        }
    }

    // Different message types
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(255, 0, 0),
    ];

    for msg in messages {
        match msg {
            Message::Quit => println!("  Quit message received"),
            Message::Move { x, y } => println!("  Move to ({}, {})", x, y),
            Message::Write(text) => println!("  Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("  Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

// Example 3: Methods on enums
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn time(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 45,
        }
    }

    fn can_go(&self) -> bool {
        match self {
            TrafficLight::Green => true,
            _ => false,
        }
    }

    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }
}

fn enum_methods() {
    let light = TrafficLight::Red;
    println!("  Light duration: {} seconds", light.time());
    println!("  Can go? {}", light.can_go());

    let next_light = light.next();
    println!("  Next light duration: {} seconds", next_light.time());
    println!("  Can go now? {}", next_light.can_go());
}

// Example 4: Option<T> - Rust's null replacement
fn option_examples() {
    // Option is defined in the standard library as:
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("  Some number: {:?}", some_number);
    println!("  Absent number: {:?}", absent_number);

    // Using match
    match some_number {
        Some(n) => println!("  Found number: {}", n),
        None => println!("  No number found"),
    }

    // Using if let
    if let Some(s) = some_string {
        println!("  String value: {}", s);
    }

    // Option methods
    let x: Option<i32> = Some(2);
    println!("  Is Some? {}", x.is_some());
    println!("  Is None? {}", x.is_none());
    println!("  Unwrap or default: {}", x.unwrap_or(0));

    // Transforming Option
    let doubled = x.map(|n| n * 2);
    println!("  Doubled: {:?}", doubled);

    // Chaining operations
    let result = Some(10)
        .map(|n| n * 2)
        .filter(|n| n > &15)
        .unwrap_or(0);
    println!("  Chained result: {}", result);
}

// Example 5: Result<T, E> - for error handling
fn result_examples() {
    // Result is defined as:
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(a / b)
        }
    }

    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);

    match result1 {
        Ok(value) => println!("  10 / 2 = {}", value),
        Err(e) => println!("  Error: {}", e),
    }

    match result2 {
        Ok(value) => println!("  Result: {}", value),
        Err(e) => println!("  Error: {}", e),
    }

    // Using unwrap_or
    let safe_division = divide(10.0, 0.0).unwrap_or(0.0);
    println!("  Safe division result: {}", safe_division);

    // Using map and unwrap_or_else
    let result = divide(20.0, 4.0)
        .map(|n| n * 2.0)
        .unwrap_or_else(|e| {
            println!("  Error occurred: {}", e);
            0.0
        });
    println!("  Mapped result: {}", result);
}

// Example 6: Complex enum variants
#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

impl WebEvent {
    fn inspect(&self) {
        match self {
            WebEvent::PageLoad => println!("    Page loaded"),
            WebEvent::PageUnload => println!("    Page unloaded"),
            WebEvent::KeyPress(c) => println!("    Key pressed: '{}'", c),
            WebEvent::Paste(s) => println!("    Pasted text: \"{}\"", s),
            WebEvent::Click { x, y } => println!("    Clicked at ({}, {})", x, y),
        }
    }
}

fn complex_variants() {
    let events = vec![
        WebEvent::PageLoad,
        WebEvent::KeyPress('x'),
        WebEvent::Paste(String::from("Hello, world!")),
        WebEvent::Click { x: 20, y: 80 },
        WebEvent::PageUnload,
    ];

    println!("  Processing web events:");
    for event in events {
        event.inspect();
    }
}

// Example 7: Enums in data structures
#[derive(Debug)]
enum Task {
    Todo(String),
    InProgress { title: String, progress: u8 },
    Done(String),
}

struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    fn new() -> Self {
        TaskList { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn summary(&self) {
        let mut todo_count = 0;
        let mut in_progress_count = 0;
        let mut done_count = 0;

        for task in &self.tasks {
            match task {
                Task::Todo(_) => todo_count += 1,
                Task::InProgress { .. } => in_progress_count += 1,
                Task::Done(_) => done_count += 1,
            }
        }

        println!("  Task Summary:");
        println!("    Todo: {}", todo_count);
        println!("    In Progress: {}", in_progress_count);
        println!("    Done: {}", done_count);
    }

    fn display(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            match task {
                Task::Todo(title) => {
                    println!("    {}. [ ] {}", i + 1, title);
                }
                Task::InProgress { title, progress } => {
                    println!("    {}. [~] {} ({}%)", i + 1, title, progress);
                }
                Task::Done(title) => {
                    println!("    {}. [✓] {}", i + 1, title);
                }
            }
        }
    }
}

fn enums_in_structures() {
    let mut task_list = TaskList::new();

    task_list.add_task(Task::Todo(String::from("Write Rust examples")));
    task_list.add_task(Task::InProgress {
        title: String::from("Learn enums"),
        progress: 75,
    });
    task_list.add_task(Task::Done(String::from("Setup Rust environment")));
    task_list.add_task(Task::Todo(String::from("Build a project")));

    println!("  Tasks:");
    task_list.display();
    println!();
    task_list.summary();
}
