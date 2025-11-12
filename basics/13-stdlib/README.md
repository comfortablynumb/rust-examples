# Standard Library

This example demonstrates commonly used features from the Rust standard library that haven't been covered in other examples, including collections, string operations, time handling, memory utilities, and common traits.

## Concepts Covered

### Collections
- **Vec** - Growable array (dynamic array)
- **HashMap** - Hash table with key-value pairs
- **HashSet** - Hash-based set of unique values
- **BTreeMap** - Ordered map (binary tree)
- **BTreeSet** - Ordered set
- **VecDeque** - Double-ended queue
- **LinkedList** - Doubly-linked list
- **BinaryHeap** - Priority queue (max heap)

### String Operations
- String creation and manipulation
- String slicing and iteration
- Common string methods (trim, split, contains, etc.)
- String formatting and parsing
- Char and byte iteration

### Time and Duration
- **Instant** - For measuring elapsed time
- **SystemTime** - Wall clock time
- **Duration** - Time span representation
- Time arithmetic and comparisons

### Memory Utilities (std::mem)
- `size_of` and `align_of` - Type sizes and alignment
- `replace` - Replace value and return old
- `swap` - Swap two values
- `take` - Take value, leaving default
- `drop` - Explicit resource cleanup
- `forget` - Leak memory (use with caution)
- `discriminant` - Enum variant comparison

### Common Traits
- **Default** - Default value construction
- **Clone** - Explicit copying
- **Copy** - Implicit bitwise copying
- **Drop** - Custom cleanup logic
- **Display** - User-facing formatting
- **Debug** - Developer-facing formatting

### Operator Overloading (std::ops)
- **Add**, **Sub**, **Mul**, **Div** - Arithmetic operators
- **Index** - Array-like indexing
- **Deref** - Dereference operator

### Conversion Traits (std::convert)
- **From** / **Into** - Infallible conversion
- **TryFrom** / **TryInto** - Fallible conversion
- **AsRef** / **AsMut** - Reference conversion

### Comparison Traits (std::cmp)
- **PartialEq** / **Eq** - Equality comparison
- **PartialOrd** / **Ord** - Ordering comparison
- **Ordering** - Comparison result

### Runtime Type Information
- **Any** - Dynamic typing
- **TypeId** - Type identification
- Downcasting

## Running the Example

```bash
cargo run
```

Run tests:
```bash
cargo test
```

## Collections Quick Reference

| Collection | Use Case | Ordered | Duplicates | Performance |
|------------|----------|---------|------------|-------------|
| `Vec<T>` | General purpose array | Insertion | Yes | O(1) push, O(1) index |
| `HashMap<K,V>` | Key-value lookup | No | No (keys) | O(1) average |
| `HashSet<T>` | Unique values | No | No | O(1) average |
| `BTreeMap<K,V>` | Ordered key-value | Yes | No (keys) | O(log n) |
| `BTreeSet<T>` | Ordered unique values | Yes | No | O(log n) |
| `VecDeque<T>` | Double-ended queue | Insertion | Yes | O(1) push/pop both ends |
| `LinkedList<T>` | Frequent inserts/removes | Insertion | Yes | O(1) insert/remove at known position |
| `BinaryHeap<T>` | Priority queue | No | Yes | O(log n) push, O(1) peek |

## String vs &str

| Type | Description | Ownership | Mutable |
|------|-------------|-----------|---------|
| `String` | Owned, growable string | Owned | Yes |
| `&str` | String slice (view) | Borrowed | No |

```rust
// String - owned, heap-allocated
let s1 = String::from("hello");
let s2 = "hello".to_string();

// &str - borrowed reference
let s3: &str = "hello";  // String literal
let s4: &str = &s1;      // Borrow from String
```

## Common Patterns

### Working with Vec

```rust
// Creation
let v1 = vec![1, 2, 3];
let v2 = Vec::new();
let v3 = Vec::with_capacity(10);

// Adding elements
v2.push(1);
v2.extend([2, 3, 4]);
v2.insert(0, 0);

// Accessing
let first = &v1[0];
let second = v1.get(1);  // Returns Option

// Iteration
for item in &v1 {
    println!("{}", item);
}

// Modification
for item in &mut v1 {
    *item += 1;
}

// Removing
v1.pop();            // Remove last
v1.remove(0);        // Remove at index
v1.retain(|x| *x > 5);  // Keep only matching
```

### Working with HashMap

```rust
use std::collections::HashMap;

// Creation
let mut map = HashMap::new();
map.insert("key", "value");

// Accessing
match map.get("key") {
    Some(value) => println!("{}", value),
    None => println!("Not found"),
}

// Updating
map.insert("key", "new_value");  // Overwrite

// Entry API (insert if missing)
map.entry("key").or_insert("default");

// Iteration
for (key, value) in &map {
    println!("{}: {}", key, value);
}

// Removing
map.remove("key");
```

### Working with String

```rust
// Creation
let s1 = String::from("hello");
let s2 = "world".to_string();

// Concatenation
let s3 = s1 + " " + &s2;  // s1 is moved
let s4 = format!("{} {}", "hello", "world");  // No moves

// Modification
let mut s = String::from("hello");
s.push_str(" world");
s.push('!');

// Common methods
s.trim();
s.to_lowercase();
s.to_uppercase();
s.replace("world", "Rust");
s.split_whitespace();
s.chars();
s.bytes();

// Slicing (be careful with UTF-8!)
let slice = &s[0..5];
```

### Time Measurement

```rust
use std::time::Instant;

let start = Instant::now();

// Do some work
expensive_operation();

let duration = start.elapsed();
println!("Took: {:?}", duration);
println!("Milliseconds: {}", duration.as_millis());
```

### Memory Utilities

```rust
use std::mem;

// Size information
mem::size_of::<i32>();
mem::align_of::<i32>();

// Moving values
let old = mem::replace(&mut value, new_value);
mem::swap(&mut x, &mut y);
let taken = mem::take(&mut value);  // Leaves default

// Resource management
mem::drop(value);    // Explicit drop
mem::forget(value);  // Leak (dangerous!)
```

### Implementing Common Traits

```rust
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Point {
    x: i32,
    y: i32,
}

// Display for user-facing output
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// From for conversions
impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point { x: tuple.0, y: tuple.1 }
    }
}

// Add for arithmetic
use std::ops::Add;

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

### Conversion Traits

```rust
use std::convert::{TryFrom, TryInto};

// From/Into (infallible)
let s = String::from("hello");
let s: String = "hello".into();

// TryFrom/TryInto (fallible)
impl TryFrom<i32> for PositiveInt {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(PositiveInt(value))
        } else {
            Err("Must be positive")
        }
    }
}

let result = PositiveInt::try_from(5)?;
```

## Best Practices

### Collections

1. **Use Vec by default** - Most versatile and performant
2. **Use HashMap for lookups** - When you need key-value pairs
3. **Use BTreeMap when order matters** - Sorted keys
4. **Prefer Vec::with_capacity** - When you know the size
5. **Use entry API** - For insert-or-update patterns
6. **Avoid LinkedList** - Rarely the best choice

### Strings

1. **Prefer &str for parameters** - More flexible than String
2. **Use String for owned data** - When you need to modify
3. **Be careful with indexing** - Use .chars() for Unicode
4. **Use format! for complex strings** - Better than concatenation
5. **Trim user input** - Remove whitespace

### Memory

1. **Let the compiler manage** - Avoid manual memory management
2. **Use mem::take** - Better than Option::take
3. **Avoid mem::forget** - Usually wrong, causes leaks
4. **Profile before optimizing** - mem::size_of for analysis

### Traits

1. **Derive when possible** - Use #[derive(...)]
2. **Implement Display** - For user-facing types
3. **Implement Debug** - Always, for debugging
4. **Use From/Into** - Better than custom conversions
5. **Implement Default** - For sensible defaults

## Performance Considerations

### Vec vs LinkedList

```rust
// Vec is almost always faster
let mut vec = Vec::new();  // ✓ Use this

// LinkedList rarely useful
let mut list = LinkedList::new();  // ✗ Avoid
```

### HashMap vs BTreeMap

```rust
// HashMap - O(1) average, unordered
let mut map = HashMap::new();  // ✓ Fast lookups

// BTreeMap - O(log n), ordered
let mut map = BTreeMap::new();  // ✓ When order matters
```

### String concatenation

```rust
// Slow - creates many intermediate strings
let s = s1 + &s2 + &s3 + &s4;  // ✗ Slow

// Fast - allocates once
let s = format!("{}{}{}{}", s1, s2, s3, s4);  // ✓ Better

// Fastest - when size known
let mut s = String::with_capacity(total_size);
s.push_str(&s1);
s.push_str(&s2);  // ✓ Best
```

## Common Errors

### Index out of bounds
```rust
let v = vec![1, 2, 3];
let x = v[10];  // Panics!

// Better: use get()
if let Some(x) = v.get(10) {
    println!("{}", x);
}
```

### String slicing
```rust
let s = "Hello 👋";
let slice = &s[0..7];  // May panic if cutting UTF-8 character!

// Better: use char boundaries
let slice = &s[0..s.char_indices().nth(6).unwrap().0];
```

### HashMap key not found
```rust
let map = HashMap::new();
let value = map["key"];  // Panics if key doesn't exist!

// Better: use get()
match map.get("key") {
    Some(value) => { /* use value */ }
    None => { /* handle missing */ }
}
```

## Resources

- [std::collections documentation](https://doc.rust-lang.org/std/collections/)
- [std::string documentation](https://doc.rust-lang.org/std/string/)
- [std::time documentation](https://doc.rust-lang.org/std/time/)
- [std::mem documentation](https://doc.rust-lang.org/std/mem/)
- [std::convert documentation](https://doc.rust-lang.org/std/convert/)
- [The Rust Standard Library book](https://doc.rust-lang.org/std/)
- [Rust by Example - Standard Library](https://doc.rust-lang.org/rust-by-example/std.html)

## When to Use External Crates

While the standard library is comprehensive, consider these crates for specialized needs:

- **serde** - Serialization/deserialization
- **chrono** - Advanced date/time handling
- **regex** - Regular expressions
- **rand** - Random number generation
- **indexmap** - HashMap with insertion order
- **smallvec** - Stack-allocated vectors
- **ahash** - Faster hashing algorithm
