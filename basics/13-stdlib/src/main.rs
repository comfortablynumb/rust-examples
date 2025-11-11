#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::approx_constant)]

// Standard Library in Rust
//
// Demonstrates commonly used standard library features including collections,
// string operations, time handling, memory utilities, and common traits.

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::fmt;
use std::ops::{Add, Deref, Index};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    println!("=== Rust Standard Library Examples ===\n");

    // Collections
    println!("1. Collections:");
    collections_examples();
    println!();

    // String operations
    println!("2. String Operations:");
    string_examples();
    println!();

    // Time and Duration
    println!("3. Time and Duration:");
    time_examples();
    println!();

    // Memory utilities
    println!("4. Memory Utilities:");
    memory_examples();
    println!();

    // Common traits
    println!("5. Common Traits:");
    trait_examples();
    println!();

    // Operator overloading
    println!("6. Operator Overloading:");
    operator_examples();
    println!();

    // Conversion traits
    println!("7. Conversion Traits:");
    conversion_examples();
    println!();

    // Comparison traits
    println!("8. Comparison Traits:");
    comparison_examples();
    println!();

    // Any trait for runtime type information
    println!("9. Runtime Type Information:");
    any_examples();
    println!();

    // Formatting
    println!("10. Custom Formatting:");
    formatting_examples();
    println!();
}

// =============================================================================
// 1. Collections Examples
// =============================================================================

fn collections_examples() {
    // Vec - growable array
    println!("  Vec (growable array):");
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    vec.extend([5, 6]);
    println!("    Vec: {:?}", vec);
    println!("    Length: {}, Capacity: {}", vec.len(), vec.capacity());

    // HashMap - hash table
    println!("\n  HashMap (hash table):");
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.entry("three").or_insert(3);
    println!("    HashMap: {:?}", map);
    println!("    Get 'two': {:?}", map.get("two"));

    // HashSet - hash-based set
    println!("\n  HashSet (unique values):");
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(1); // Duplicate, won't be added
    println!("    HashSet: {:?}", set);
    println!("    Contains 1: {}", set.contains(&1));

    // BTreeMap - ordered map
    println!("\n  BTreeMap (ordered map):");
    let mut btree: BTreeMap<i32, &str> = BTreeMap::new();
    btree.insert(3, "three");
    btree.insert(1, "one");
    btree.insert(2, "two");
    println!("    BTreeMap (ordered): {:?}", btree);

    // BTreeSet - ordered set
    println!("\n  BTreeSet (ordered set):");
    let mut btreeset: BTreeSet<i32> = BTreeSet::new();
    btreeset.insert(5);
    btreeset.insert(1);
    btreeset.insert(3);
    println!("    BTreeSet (ordered): {:?}", btreeset);

    // VecDeque - double-ended queue
    println!("\n  VecDeque (double-ended queue):");
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    println!("    VecDeque: {:?}", deque);
    println!("    Pop front: {:?}", deque.pop_front());

    // LinkedList - doubly-linked list
    println!("\n  LinkedList:");
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(0);
    println!("    LinkedList: {:?}", list);

    // BinaryHeap - priority queue (max heap)
    println!("\n  BinaryHeap (priority queue):");
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(5);
    heap.push(2);
    println!("    BinaryHeap: {:?}", heap);
    println!("    Pop max: {:?}", heap.pop());
    println!("    Peek max: {:?}", heap.peek());
}

// =============================================================================
// 2. String Operations
// =============================================================================

fn string_examples() {
    // String creation
    println!("  String creation:");
    let s1 = String::from("Hello");
    let s2 = "World".to_string();
    let s3 = String::new();
    println!("    s1: {}, s2: {}, s3 is empty: {}", s1, s2, s3.is_empty());

    // String manipulation
    println!("\n  String manipulation:");
    let mut s = String::from("Hello");
    s.push_str(", World");
    s.push('!');
    println!("    After push: {}", s);

    // String slicing
    println!("\n  String slicing:");
    let s = "Hello, World!";
    let hello = &s[0..5];
    let world = &s[7..12];
    println!("    First part: {}, Second part: {}", hello, world);

    // String iteration
    println!("\n  String iteration:");
    let s = "Hello";
    print!("    Chars: ");
    for c in s.chars() {
        print!("{} ", c);
    }
    println!();

    print!("    Bytes: ");
    for b in s.bytes() {
        print!("{} ", b);
    }
    println!();

    // String methods
    println!("\n  String methods:");
    let s = "  Hello, World!  ";
    println!("    Original: '{}'", s);
    println!("    Trimmed: '{}'", s.trim());
    println!("    Uppercase: '{}'", s.to_uppercase());
    println!("    Lowercase: '{}'", s.to_lowercase());
    println!("    Contains 'World': {}", s.contains("World"));
    println!("    Starts with '  Hello': {}", s.starts_with("  Hello"));
    println!("    Ends with '!  ': {}", s.ends_with("!  "));

    // String splitting
    println!("\n  String splitting:");
    let s = "one,two,three";
    let parts: Vec<&str> = s.split(',').collect();
    println!("    Split by ',': {:?}", parts);

    // String formatting
    println!("\n  String formatting:");
    let name = "Alice";
    let age = 30;
    let formatted = format!("Name: {}, Age: {}", name, age);
    println!("    Formatted: {}", formatted);

    // String parsing
    println!("\n  String parsing:");
    let num_str = "42";
    let num: i32 = num_str.parse().unwrap();
    println!("    Parsed '{}' to integer: {}", num_str, num);

    let float_str = "3.14";
    let float: f64 = float_str.parse().unwrap();
    println!("    Parsed '{}' to float: {}", float_str, float);
}

// =============================================================================
// 3. Time and Duration Examples
// =============================================================================

fn time_examples() {
    // Instant for measuring elapsed time
    println!("  Instant (for timing):");
    let start = Instant::now();
    // Simulate some work
    let mut sum = 0;
    for i in 0..1000 {
        sum += i;
    }
    let elapsed = start.elapsed();
    println!("    Operation took: {:?}", elapsed);
    println!("    As milliseconds: {}", elapsed.as_millis());
    println!("    As microseconds: {}", elapsed.as_micros());

    // SystemTime for wall clock time
    println!("\n  SystemTime (wall clock):");
    let now = SystemTime::now();
    println!("    Current time: {:?}", now);

    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    println!("    Seconds since Unix epoch: {}", since_epoch.as_secs());

    // Duration creation and manipulation
    println!("\n  Duration:");
    let d1 = Duration::from_secs(5);
    let d2 = Duration::from_millis(500);
    let d3 = Duration::from_micros(1_000_000);

    println!("    5 seconds: {:?}", d1);
    println!("    500 milliseconds: {:?}", d2);
    println!("    1 million microseconds: {:?}", d3);

    let total = d1 + d2;
    println!("    5s + 500ms = {:?}", total);

    // Duration comparisons
    println!("\n  Duration comparisons:");
    let short = Duration::from_millis(100);
    let long = Duration::from_secs(1);
    println!("    100ms < 1s: {}", short < long);
}

// =============================================================================
// 4. Memory Utilities
// =============================================================================

fn memory_examples() {
    use std::mem;

    // size_of and align_of
    println!("  Size and alignment:");
    println!("    size_of::<i32>() = {} bytes", mem::size_of::<i32>());
    println!("    size_of::<i64>() = {} bytes", mem::size_of::<i64>());
    println!("    size_of::<String>() = {} bytes", mem::size_of::<String>());
    println!("    align_of::<i32>() = {} bytes", mem::align_of::<i32>());

    // replace
    println!("\n  mem::replace:");
    let mut value = 5;
    let old = mem::replace(&mut value, 10);
    println!("    Old value: {}, New value: {}", old, value);

    // swap
    println!("\n  mem::swap:");
    let mut x = 1;
    let mut y = 2;
    println!("    Before swap: x={}, y={}", x, y);
    mem::swap(&mut x, &mut y);
    println!("    After swap: x={}, y={}", x, y);

    // take (replaces with default)
    println!("\n  mem::take:");
    let mut s = String::from("Hello");
    let taken = mem::take(&mut s);
    println!("    Taken: '{}', Original is now: '{}'", taken, s);

    // drop
    println!("\n  mem::drop:");
    let s = String::from("will be dropped");
    println!("    Before drop: '{}'", s);
    mem::drop(s);
    println!("    After drop (s is no longer accessible)");

    // forget (leak memory - use with caution!)
    println!("\n  mem::forget (leaks memory):");
    let s = String::from("will be leaked");
    mem::forget(s);
    println!("    String was forgotten (memory leaked)");

    // discriminant for enums
    println!("\n  mem::discriminant:");
    #[derive(Debug)]
    enum MyEnum {
        A,
        B(i32),
        C(String),
    }
    let e1 = MyEnum::A;
    let e2 = MyEnum::B(10);
    let e3 = MyEnum::B(20);
    println!(
        "    A and B same variant: {}",
        mem::discriminant(&e1) == mem::discriminant(&e2)
    );
    println!(
        "    B(10) and B(20) same variant: {}",
        mem::discriminant(&e2) == mem::discriminant(&e3)
    );
}

// =============================================================================
// 5. Common Traits Examples
// =============================================================================

fn trait_examples() {
    // Default trait
    println!("  Default trait:");
    #[derive(Debug, Default)]
    struct Config {
        timeout: u32,
        retries: u32,
    }
    let config = Config::default();
    println!("    Default config: {:?}", config);

    // Clone trait
    println!("\n  Clone trait:");
    let vec1 = vec![1, 2, 3];
    let vec2 = vec1.clone();
    println!("    Original: {:?}, Cloned: {:?}", vec1, vec2);

    // Copy trait (implicit, for types that can be copied bitwise)
    println!("\n  Copy trait:");
    let x = 5;
    let y = x; // Copy happens here
    println!("    x: {}, y: {} (both usable)", x, y);

    // Drop trait
    println!("\n  Drop trait:");
    struct Resource {
        name: String,
    }
    impl Drop for Resource {
        fn drop(&mut self) {
            println!("    Dropping resource: {}", self.name);
        }
    }
    {
        let _r = Resource {
            name: "MyResource".to_string(),
        };
        println!("    Resource created");
    }
    println!("    Resource dropped when out of scope");
}

// =============================================================================
// 6. Operator Overloading Examples
// =============================================================================

fn operator_examples() {
    // Custom type with operator overloading
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Index<usize> for Point {
        type Output = i32;

        fn index(&self, idx: usize) -> &i32 {
            match idx {
                0 => &self.x,
                1 => &self.y,
                _ => panic!("Index out of bounds"),
            }
        }
    }

    println!("  Custom Point with Add:");
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("    {:?} + {:?} = {:?}", p1, p2, p3);

    println!("\n  Custom Point with Index:");
    println!("    p1[0] = {}, p1[1] = {}", p1[0], p1[1]);

    // Deref example
    println!("\n  Deref trait:");
    struct MyBox<T>(T);

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    let boxed = MyBox(String::from("Hello"));
    println!("    Deref MyBox: {}", *boxed);
    println!("    Method call: {}", boxed.len()); // Deref coercion
}

// =============================================================================
// 7. Conversion Traits Examples
// =============================================================================

fn conversion_examples() {
    // From and Into
    println!("  From and Into:");

    #[derive(Debug)]
    struct Celsius(f64);

    #[derive(Debug)]
    struct Fahrenheit(f64);

    impl From<Celsius> for Fahrenheit {
        fn from(c: Celsius) -> Fahrenheit {
            Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
        }
    }

    let c = Celsius(25.0);
    let f: Fahrenheit = c.into(); // Into is automatically implemented
    println!("    {:?} = {:?}", Celsius(25.0), f);

    // TryFrom and TryInto
    println!("\n  TryFrom and TryInto:");
    use std::convert::TryFrom;

    #[derive(Debug)]
    struct PositiveNumber(i32);

    impl TryFrom<i32> for PositiveNumber {
        type Error = &'static str;

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value > 0 {
                Ok(PositiveNumber(value))
            } else {
                Err("Number must be positive")
            }
        }
    }

    let pos = PositiveNumber::try_from(5);
    let neg = PositiveNumber::try_from(-5);
    println!("    Try from 5: {:?}", pos);
    println!("    Try from -5: {:?}", neg);

    // AsRef and AsMut
    println!("\n  AsRef:");
    fn print_length<T: AsRef<str>>(s: T) {
        println!("    Length: {}", s.as_ref().len());
    }
    print_length("hello");
    print_length(String::from("world"));
}

// =============================================================================
// 8. Comparison Traits Examples
// =============================================================================

fn comparison_examples() {
    use std::cmp::Ordering;

    // Custom type with Ord
    #[derive(Debug, Eq, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    impl PartialOrd for Person {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Person {
        fn cmp(&self, other: &Self) -> Ordering {
            self.age.cmp(&other.age)
        }
    }

    println!("  Custom ordering (by age):");
    let alice = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    let bob = Person {
        name: "Bob".to_string(),
        age: 25,
    };

    println!("    {:?}", alice);
    println!("    {:?}", bob);
    println!("    Alice > Bob: {}", alice > bob);
    println!("    Ordering: {:?}", alice.cmp(&bob));

    // Min/Max
    println!("\n  Min/Max:");
    use std::cmp::{max, min};
    println!("    min(5, 10) = {}", min(5, 10));
    println!("    max(5, 10) = {}", max(5, 10));
}

// =============================================================================
// 9. Any Trait (Runtime Type Information)
// =============================================================================

fn any_examples() {
    use std::any::{Any, TypeId};

    println!("  Any trait for runtime type info:");

    fn print_type_info<T: Any>(value: &T) {
        println!("    Type ID: {:?}", value.type_id());
        println!("    Type name: {}", std::any::type_name::<T>());
    }

    print_type_info(&42i32);
    print_type_info(&"hello");
    print_type_info(&vec![1, 2, 3]);

    // Downcasting
    println!("\n  Downcasting with Any:");
    let value: &dyn Any = &42i32;

    if let Some(num) = value.downcast_ref::<i32>() {
        println!("    Successfully downcast to i32: {}", num);
    }

    if value.downcast_ref::<String>().is_none() {
        println!("    Cannot downcast to String");
    }
}

// =============================================================================
// 10. Custom Formatting Examples
// =============================================================================

fn formatting_examples() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} (age {})", self.name, self.age)
        }
    }

    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    println!("  Display: {}", person);
    println!("  Debug: {:?}", person);

    // Formatting options
    println!("\n  Formatting options:");
    let num = 42;
    println!("    Decimal: {}", num);
    println!("    Binary: {:b}", num);
    println!("    Octal: {:o}", num);
    println!("    Hexadecimal: {:x}", num);
    println!("    Hexadecimal (upper): {:X}", num);

    let float = 3.14159;
    println!("    Float: {}", float);
    println!("    Float (2 decimals): {:.2}", float);
    println!("    Float (scientific): {:e}", float);

    let text = "hello";
    println!("    Text: {}", text);
    println!("    Text (width 10): '{:10}'", text);
    println!("    Text (width 10, right): '{:>10}'", text);
    println!("    Text (width 10, center): '{:^10}'", text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collections() {
        let mut vec = vec![1, 2, 3];
        vec.push(4);
        assert_eq!(vec.len(), 4);

        let mut map = HashMap::new();
        map.insert("key", "value");
        assert_eq!(map.get("key"), Some(&"value"));
    }

    #[test]
    fn test_string_operations() {
        let s = String::from("Hello");
        assert_eq!(s.len(), 5);
        assert!(s.contains("ell"));

        let parts: Vec<&str> = "a,b,c".split(',').collect();
        assert_eq!(parts, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_duration() {
        let d1 = Duration::from_secs(1);
        let d2 = Duration::from_millis(500);
        let total = d1 + d2;
        assert_eq!(total.as_millis(), 1500);
    }

    #[test]
    fn test_memory_utilities() {
        use std::mem;

        assert_eq!(mem::size_of::<i32>(), 4);
        assert_eq!(mem::size_of::<i64>(), 8);

        let mut x = 5;
        let old = mem::replace(&mut x, 10);
        assert_eq!(old, 5);
        assert_eq!(x, 10);
    }
}
