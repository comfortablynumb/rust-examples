# Heapless Collections

This example demonstrates using the `heapless` crate to work with data structures without dynamic memory allocation.

## Why Heapless?

In embedded systems and bare metal environments, dynamic memory allocation can be problematic:

### Problems with Heap Allocation

1. **Unpredictable timing**: `malloc`/`free` can take variable time
2. **Fragmentation**: Memory can become fragmented over time
3. **Failure**: Allocation can fail (out of memory)
4. **Complexity**: Requires allocator implementation
5. **Code size**: Heap allocator adds significant code
6. **No guarantees**: Hard to prove memory safety

### Benefits of Heapless

- ✅ **Predictable**: All memory on stack/static, compile-time known
- ✅ **Fast**: No allocation overhead, cache-friendly
- ✅ **Safe**: Cannot run out of memory unexpectedly
- ✅ **Small**: No allocator code needed
- ✅ **Deterministic**: Perfect for real-time systems
- ✅ **Provable**: Easy to reason about memory usage

## Collection Types

### 1. Vec<T, N>

Fixed-capacity vector (like `std::vec::Vec`):

```rust
use heapless::Vec;

// Vec with capacity for 16 elements
let mut vec: Vec<i32, 16> = Vec::new();

// Push elements
vec.push(1).unwrap();
vec.push(2).unwrap();

// Iterate
for item in &vec {
    println!("{}", item);
}

// Pop elements
let last = vec.pop();

// Check capacity
assert_eq!(vec.capacity(), 16);
```

**Use cases:**
- Sensor reading buffers
- Command queues
- Data packet storage
- Any list with known maximum size

### 2. String<N>

Fixed-capacity string (like `std::string::String`):

```rust
use heapless::String;

// String with capacity for 32 bytes
let mut s: String<32> = String::new();

// Push characters and strings
s.push('H').unwrap();
s.push_str("ello").unwrap();

// Format into string
use core::fmt::Write;
write!(&mut s, " {}", 42).unwrap();

// Convert to &str
let as_str: &str = &s;
```

**Use cases:**
- Log messages
- Status strings
- Serial communication
- Display text

### 3. Deque<T, N>

Double-ended queue with ring buffer:

```rust
use heapless::Deque;

let mut deque: Deque<u8, 32> = Deque::new();

// Push/pop from either end
deque.push_back(1).unwrap();   // [1]
deque.push_front(0).unwrap();  // [0, 1]
deque.push_back(2).unwrap();   // [0, 1, 2]

let front = deque.pop_front(); // Some(0)
let back = deque.pop_back();   // Some(2)

// Use as FIFO queue
deque.push_back(msg).unwrap();  // Enqueue
let msg = deque.pop_front();     // Dequeue

// Use as LIFO stack
deque.push_back(item).unwrap();  // Push
let item = deque.pop_back();     // Pop
```

**Use cases:**
- Message queues
- Event buffers
- Undo/redo stacks
- Sliding windows

### 4. IndexMap<K, V, N>

Fixed-capacity hash map (like `std::collections::HashMap`):

```rust
use heapless::FnvIndexMap;

// Map with capacity for 8 entries
let mut map: FnvIndexMap<&str, u32, 8> = FnvIndexMap::new();

// Insert
map.insert("temperature", 25).unwrap();
map.insert("humidity", 60).unwrap();

// Get
if let Some(&temp) = map.get("temperature") {
    // Use temp
}

// Update
if let Some(value) = map.get_mut("temperature") {
    *value = 26;
}

// Remove
map.remove("temperature");
```

**Use cases:**
- Configuration storage
- Sensor name to value mapping
- Command lookup tables
- Caching

### 5. HistoryBuffer<T, N>

Circular buffer that keeps last N items:

```rust
use heapless::HistoryBuffer;

let mut history: HistoryBuffer<u32, 16> = HistoryBuffer::new();

// Write values (overwrites oldest)
for i in 0..20 {
    history.write(i);
}

// Buffer contains last 16 values: [4..20]

// Calculate moving average
let sum: u32 = history.recent().sum();
let avg = sum / history.len() as u32;

// Access newest to oldest
for &value in history.recent() {
    // Process value
}

// Access oldest to newest
for &value in history.oldest_ordered() {
    // Process value
}
```

**Use cases:**
- Moving averages
- Signal filtering
- Recent event history
- Time-series data

### 6. Queue (MPMC/SPSC)

Lock-free queues for inter-thread/ISR communication:

```rust
use heapless::mpmc::Q8;

static QUEUE: Q8<SensorReading> = Q8::new();

// In interrupt handler (producer)
#[interrupt]
fn SENSOR_IRQ() {
    let reading = read_sensor();
    QUEUE.enqueue(reading).ok();
}

// In main loop (consumer)
fn main() -> ! {
    loop {
        if let Some(reading) = QUEUE.dequeue() {
            process(reading);
        }
    }
}
```

**Queue types:**
- `mpmc::QN<T>`: Multi-Producer Multi-Consumer
- `spsc::Queue<T, N>`: Single-Producer Single-Consumer (faster)

**Use cases:**
- ISR to main thread communication
- Producer-consumer patterns
- Task scheduling
- Event dispatching

## Error Handling

All operations that can fail return `Result` or `Option`:

```rust
use heapless::Vec;

let mut vec: Vec<u8, 4> = Vec::new();

// Fill the vec
for i in 0..4 {
    vec.push(i).unwrap();
}

// This will fail
match vec.push(5) {
    Ok(_) => println!("Added"),
    Err(_) => println!("Vec is full!"),
}
```

### Handling Full Containers

```rust
// Check before pushing
if !vec.is_full() {
    vec.push(item).ok();
}

// Use Result
if let Err(item) = vec.push(item) {
    // Vec was full, item was returned
    handle_overflow(item);
}

// Overwrite oldest (for circular buffers)
history.write(new_value); // Automatically overwrites
```

## Memory Usage

All memory is known at compile time:

```rust
use core::mem::size_of;
use heapless::{Vec, String};

// Vec<u32, 16> size
let vec_size = size_of::<Vec<u32, 16>>();
// = 16 elements * 4 bytes + metadata
// ≈ 72 bytes

// String<64> size
let string_size = size_of::<String<64>>();
// = 64 bytes + length
// ≈ 72 bytes
```

## Common Patterns

### Ring Buffer for Streaming Data

```rust
use heapless::Deque;

let mut buffer: Deque<u8, 1024> = Deque::new();

loop {
    // Add new data
    if let Some(byte) = uart_read() {
        if buffer.is_full() {
            buffer.pop_front(); // Discard oldest
        }
        buffer.push_back(byte).ok();
    }

    // Process when enough data
    if buffer.len() >= 10 {
        let data: Vec<u8, 10> = buffer
            .iter()
            .take(10)
            .cloned()
            .collect();
        process_packet(&data);
    }
}
```

### State Machine with Fixed States

```rust
use heapless::FnvIndexMap;

type StateFn = fn() -> State;

let mut states: FnvIndexMap<State, StateFn, 8> = FnvIndexMap::new();

states.insert(State::Init, init_state).unwrap();
states.insert(State::Running, running_state).unwrap();
states.insert(State::Error, error_state).unwrap();

let mut current = State::Init;

loop {
    if let Some(&state_fn) = states.get(&current) {
        current = state_fn();
    }
}
```

### Moving Average Filter

```rust
use heapless::HistoryBuffer;

let mut readings: HistoryBuffer<i16, 10> = HistoryBuffer::new();

fn read_filtered_sensor() -> i16 {
    let raw = read_sensor();
    readings.write(raw);

    let sum: i32 = readings.recent().map(|&x| x as i32).sum();
    (sum / readings.len() as i32) as i16
}
```

### Message Queue

```rust
use heapless::Deque;

#[derive(Clone)]
struct Message {
    id: u8,
    data: [u8; 16],
}

static mut MSG_QUEUE: Deque<Message, 32> = Deque::new();

fn send_message(msg: Message) -> Result<(), Message> {
    unsafe { MSG_QUEUE.push_back(msg) }
}

fn receive_message() -> Option<Message> {
    unsafe { MSG_QUEUE.pop_front() }
}
```

### Memory Pool

```rust
struct Pool<T, const N: usize> {
    storage: [Option<T>; N],
}

impl<T, const N: usize> Pool<T, N> {
    const fn new() -> Self {
        Self {
            storage: [None; N],
        }
    }

    fn alloc(&mut self, value: T) -> Option<usize> {
        for (i, slot) in self.storage.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(value);
                return Some(i);
            }
        }
        None
    }

    fn free(&mut self, index: usize) -> Option<T> {
        self.storage.get_mut(index)?.take()
    }
}

static mut BUFFER_POOL: Pool<[u8; 128], 8> = Pool::new();
```

## Performance Characteristics

| Operation | Vec | Deque | IndexMap | HistoryBuffer |
|-----------|-----|-------|----------|---------------|
| Push/Insert | O(1) | O(1) | O(1)* | O(1) |
| Pop/Remove | O(1) | O(1) | O(n) | - |
| Access | O(1) | O(1) | O(1)* | O(1) |
| Search | O(n) | O(n) | O(1)* | O(n) |

*Average case, assumes good hash function

## Integration with Embedded HAL

```rust
use embedded_hal::serial::Read;
use heapless::{Vec, String};

fn read_line<S: Read<u8>>(serial: &mut S) -> Result<String<64>, ()> {
    let mut line: String<64> = String::new();

    loop {
        match serial.read() {
            Ok(b'\n') => return Ok(line),
            Ok(byte) => {
                line.push(byte as char).map_err(|_| ())?;
            }
            Err(_) => {}
        }
    }
}
```

## Building

```bash
# Build for any no_std target
cargo build --target thumbv7em-none-eabihf

# Check size
cargo size --release

# Run tests (requires std)
cargo test
```

## Choosing Capacity

Guidelines for choosing N:

1. **Overestimate slightly**: Better too much than too little
2. **Power of 2**: Often more efficient (especially for queues)
3. **Profile**: Measure actual usage in testing
4. **Consider worst case**: Peak usage, not average
5. **Memory budget**: Check total memory usage fits in RAM

Example sizing:

```rust
// Log messages: max 80 chars
type LogMessage = String<80>;

// Sensor readings: 100Hz, 1 second buffer
type SensorBuffer = HistoryBuffer<Reading, 100>;

// Command queue: max 8 pending commands
type CommandQueue = Deque<Command, 8>;

// Configuration: max 16 settings
type Config = FnvIndexMap<&'static str, u32, 16>;
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use heapless::Vec;

    #[test]
    fn test_vec_full() {
        let mut vec: Vec<u8, 4> = Vec::new();

        // Fill vec
        for i in 0..4 {
            assert!(vec.push(i).is_ok());
        }

        // Next push should fail
        assert!(vec.push(5).is_err());
        assert!(vec.is_full());
    }
}
```

## Comparison with Standard Library

| std | heapless | Differences |
|-----|----------|-------------|
| `Vec<T>` | `Vec<T, N>` | Fixed capacity N |
| `String` | `String<N>` | Fixed capacity N |
| `VecDeque<T>` | `Deque<T, N>` | Fixed capacity N |
| `HashMap<K, V>` | `IndexMap<K, V, N>` | Fixed capacity N, different hash |
| - | `HistoryBuffer<T, N>` | Circular buffer |
| `mpsc::channel()` | `mpmc::QN<T>` | Lock-free, bounded |

## References

- [heapless crate docs](https://docs.rs/heapless/)
- [The Embedded Rust Book - Collections](https://rust-embedded.github.io/book/collections/)
- [Static Guarantees](https://rust-embedded.github.io/book/static-guarantees/)
