#![allow(dead_code)]
#![allow(unused_variables)]

//! # Traits and Generics in Rust
//!
//! This example demonstrates comprehensive usage of traits and generics in Rust,
//! including advanced patterns and real-world applications.

use std::fmt::{Debug, Display};

// ============================================================================
// 1. GENERIC FUNCTIONS AND STRUCTS
// ============================================================================

/// Generic function that works with any type implementing Display
fn print_value<T: Display>(value: T) {
    println!("Value: {}", value);
}

/// Generic function with multiple type parameters
fn print_pair<T: Display, U: Display>(first: T, second: U) {
    println!("Pair: ({}, {})", first, second);
}

/// Generic struct with a single type parameter
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }

    fn get(&self) -> &T {
        &self.value
    }
}

/// Generic struct with multiple type parameters
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }

    fn into_tuple(self) -> (T, U) {
        (self.first, self.second)
    }
}

/// Generic enum example
enum Result2<T, E> {
    Success(T),
    Failure(E),
}

// ============================================================================
// 2. TRAIT BOUNDS
// ============================================================================

/// Function with trait bounds in angle brackets
fn compare_and_display<T: PartialOrd + Display>(a: T, b: T) {
    if a > b {
        println!("{} is greater than {}", a, b);
    } else {
        println!("{} is not greater than {}", a, b);
    }
}

/// Generic struct with trait bounds
struct Comparator<T: PartialOrd> {
    value: T,
}

impl<T: PartialOrd> Comparator<T> {
    fn new(value: T) -> Self {
        Comparator { value }
    }

    fn is_greater_than(&self, other: &T) -> bool {
        self.value > *other
    }
}

/// Multiple trait bounds example
fn process_data<T: Clone + Debug + Display>(data: T) {
    let cloned = data.clone();
    println!("Debug: {:?}", cloned);
    println!("Display: {}", data);
}

// ============================================================================
// 3. WHERE CLAUSES
// ============================================================================

/// Using where clause for better readability
fn complex_function<T, U, V>(a: T, b: U, c: V)
where
    T: Display + Clone,
    U: Debug + PartialEq,
    V: Into<String>,
{
    println!("T: {}", a);
    println!("U: {:?}", b);
    println!("V: {}", c.into());
}

/// Where clause with lifetime constraints
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// Complex where clause with multiple constraints
struct ComplexStruct<T, U> {
    data: Vec<T>,
    metadata: U,
}

impl<T, U> ComplexStruct<T, U>
where
    T: Clone + Debug,
    U: Display + Default,
{
    fn new() -> Self {
        ComplexStruct {
            data: Vec::new(),
            metadata: U::default(),
        }
    }

    fn add_item(&mut self, item: T) {
        self.data.push(item);
    }
}

// ============================================================================
// 4. ASSOCIATED TYPES
// ============================================================================

/// Trait with associated type
trait Iterator2 {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

/// Implementation with concrete associated type
struct Counter {
    count: u32,
    max: u32,
}

impl Iterator2 for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/// Generic trait with associated types
trait Graph {
    type Node;
    type Edge;

    fn has_edge(&self, from: &Self::Node, to: &Self::Node) -> bool;
    fn edges(&self, node: &Self::Node) -> Vec<Self::Edge>;
}

// ============================================================================
// 5. GENERIC TRAITS
// ============================================================================

/// Generic trait that can be implemented for different types
trait Converter<T> {
    fn convert(&self) -> T;
}

struct IntWrapper(i32);

impl Converter<String> for IntWrapper {
    fn convert(&self) -> String {
        self.0.to_string()
    }
}

impl Converter<f64> for IntWrapper {
    fn convert(&self) -> f64 {
        self.0 as f64
    }
}

/// Generic trait with multiple type parameters
trait Combiner<T, U> {
    type Output;

    fn combine(&self, other: T) -> Self::Output;
}

impl Combiner<i32, ()> for String {
    type Output = String;

    fn combine(&self, other: i32) -> Self::Output {
        format!("{}-{}", self, other)
    }
}

// ============================================================================
// 6. TRAIT OBJECTS (dyn Trait)
// ============================================================================

/// Object-safe trait for dynamic dispatch
trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle {}x{}", self.width, self.height);
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

/// Function that accepts trait objects
fn draw_shapes(shapes: &[Box<dyn Drawable>]) {
    for shape in shapes {
        shape.draw();
        println!("Area: {}", shape.area());
    }
}

/// Trait object as return type
fn create_shape(shape_type: &str) -> Box<dyn Drawable> {
    match shape_type {
        "circle" => Box::new(Circle { radius: 5.0 }),
        "rectangle" => Box::new(Rectangle {
            width: 4.0,
            height: 6.0,
        }),
        _ => Box::new(Circle { radius: 1.0 }),
    }
}

// ============================================================================
// 7. STATIC VS DYNAMIC DISPATCH
// ============================================================================

/// Static dispatch - monomorphization at compile time
fn static_dispatch<T: Drawable>(shape: &T) {
    shape.draw();
}

/// Dynamic dispatch - runtime polymorphism
fn dynamic_dispatch(shape: &dyn Drawable) {
    shape.draw();
}

/// Comparison example
struct Triangle {
    base: f64,
    height: f64,
}

impl Drawable for Triangle {
    fn draw(&self) {
        println!(
            "Drawing a triangle with base {} and height {}",
            self.base, self.height
        );
    }

    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

fn demonstrate_dispatch() {
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 5.0,
    };

    // Static dispatch - compiler knows exact types
    static_dispatch(&circle);
    static_dispatch(&rectangle);

    // Dynamic dispatch - type determined at runtime
    let shapes: Vec<&dyn Drawable> = vec![&circle, &rectangle];
    for shape in shapes {
        dynamic_dispatch(shape);
    }
}

// ============================================================================
// 8. IMPL TRAIT
// ============================================================================

/// Return impl Trait - static dispatch without naming the type
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

/// impl Trait in argument position
fn print_iter(iter: impl Iterator<Item = i32>) {
    for item in iter {
        println!("{}", item);
    }
}

/// Combining impl Trait with trait bounds
fn create_iterator(start: i32, end: i32) -> impl Iterator<Item = i32> + Clone {
    (start..end).into_iter()
}

/// Multiple impl Trait returns
fn create_closures() -> (impl Fn(i32) -> i32, impl Fn(i32) -> i32) {
    let add_one = |x| x + 1;
    let multiply_two = |x| x * 2;
    (add_one, multiply_two)
}

/// impl Trait with complex constraints
fn process_collection<T>(collection: T) -> impl Iterator<Item = String>
where
    T: IntoIterator,
    T::Item: Display,
{
    collection.into_iter().map(|item| format!("Item: {}", item))
}

// ============================================================================
// 9. TRAIT INHERITANCE
// ============================================================================

/// Base trait
trait Animal {
    fn name(&self) -> &str;
    fn make_sound(&self);
}

/// Trait that extends Animal
trait Mammal: Animal {
    fn has_fur(&self) -> bool;
}

/// Trait that extends multiple traits
trait Pet: Animal + Clone {
    fn owner(&self) -> &str;
}

#[derive(Clone)]
struct Dog {
    name: String,
    owner: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }

    fn make_sound(&self) {
        println!("Woof!");
    }
}

impl Mammal for Dog {
    fn has_fur(&self) -> bool {
        true
    }
}

impl Pet for Dog {
    fn owner(&self) -> &str {
        &self.owner
    }
}

/// Function requiring trait inheritance
fn pet_info<T: Pet>(pet: &T) {
    println!("Pet name: {}", pet.name());
    println!("Owner: {}", pet.owner());
    pet.make_sound();
}

// ============================================================================
// 10. BLANKET IMPLEMENTATIONS
// ============================================================================

/// Custom trait for demonstration
trait Printable {
    fn print(&self);
}

/// Blanket implementation for all types implementing Display
impl<T: Display> Printable for T {
    fn print(&self) {
        println!("Printing: {}", self);
    }
}

/// Another blanket implementation example
trait AsBytes {
    fn as_bytes_vec(&self) -> Vec<u8>;
}

/// Implement for all types that can be converted to String
impl<T: ToString> AsBytes for T {
    fn as_bytes_vec(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

/// Blanket implementation with additional constraints
trait Summarizable {
    fn summary(&self) -> String;
}

impl<T: Debug + Display> Summarizable for T {
    fn summary(&self) -> String {
        format!("Debug: {:?}, Display: {}", self, self)
    }
}

// ============================================================================
// 11. MARKER TRAITS (Send, Sync, Copy, Sized)
// ============================================================================

/// Custom marker trait
trait Trusted {}

struct SafeData {
    value: i32,
}

impl Trusted for SafeData {}

/// Function that requires Send trait
fn process_in_thread<T: Send + 'static>(data: T) -> std::thread::JoinHandle<T> {
    std::thread::spawn(move || {
        // Process data
        data
    })
}

/// Function that requires Sync trait
fn share_across_threads<T: Sync>(data: &T) {
    // Can safely share reference across threads
    println!("Data is Sync-safe");
}

/// Working with Copy trait
fn demonstrate_copy<T: Copy>(value: T) -> (T, T) {
    (value, value) // T is copied, not moved
}

/// Sized trait examples
fn print_sized<T: Sized>(value: T) {
    println!("Size known at compile time");
}

/// Relaxing Sized bound with ?Sized
fn print_maybe_sized<T: ?Sized + Display>(value: &T) {
    println!("Value: {}", value);
}

/// Custom marker trait for compile-time guarantees
trait Serializable {}
trait Deserializable {}

struct JsonData {
    content: String,
}

impl Serializable for JsonData {}
impl Deserializable for JsonData {}

fn save_data<T: Serializable>(_data: T) {
    println!("Saving data...");
}

fn load_data<T: Deserializable>(_data: T) {
    println!("Loading data...");
}

// ============================================================================
// 12. ADVANCED PATTERNS
// ============================================================================

// NEWTYPE PATTERN
/// Newtype pattern for type safety
struct Meters(f64);
struct Kilometers(f64);

impl Meters {
    fn to_kilometers(&self) -> Kilometers {
        Kilometers(self.0 / 1000.0)
    }
}

impl Kilometers {
    fn to_meters(&self) -> Meters {
        Meters(self.0 * 1000.0)
    }
}

/// Newtype with trait implementation
struct Username(String);

impl Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}

// TYPE STATE PATTERN
/// Type state pattern for compile-time state verification
struct Locked;
struct Unlocked;

struct Door<State> {
    _state: std::marker::PhantomData<State>,
}

impl Door<Locked> {
    fn new() -> Self {
        println!("Creating a locked door");
        Door {
            _state: std::marker::PhantomData,
        }
    }

    fn unlock(self) -> Door<Unlocked> {
        println!("Unlocking door");
        Door {
            _state: std::marker::PhantomData,
        }
    }
}

impl Door<Unlocked> {
    fn lock(self) -> Door<Locked> {
        println!("Locking door");
        Door {
            _state: std::marker::PhantomData,
        }
    }

    fn open(&self) {
        println!("Opening door");
    }
}

/// Builder pattern with type state
struct Empty;
struct WithName;
struct WithAge;

struct PersonBuilder<State> {
    name: Option<String>,
    age: Option<u32>,
    _state: std::marker::PhantomData<State>,
}

impl PersonBuilder<Empty> {
    fn new() -> Self {
        PersonBuilder {
            name: None,
            age: None,
            _state: std::marker::PhantomData,
        }
    }

    fn name(mut self, name: String) -> PersonBuilder<WithName> {
        self.name = Some(name);
        PersonBuilder {
            name: self.name,
            age: self.age,
            _state: std::marker::PhantomData,
        }
    }
}

impl PersonBuilder<WithName> {
    fn age(mut self, age: u32) -> PersonBuilder<WithAge> {
        self.age = Some(age);
        PersonBuilder {
            name: self.name,
            age: self.age,
            _state: std::marker::PhantomData,
        }
    }
}

impl PersonBuilder<WithAge> {
    fn build(self) -> Person {
        Person {
            name: self.name.unwrap(),
            age: self.age.unwrap(),
        }
    }
}

struct Person {
    name: String,
    age: u32,
}

/// Phantom data for zero-cost abstractions
struct Slice<'a, T> {
    data: &'a [T],
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T> Slice<'a, T> {
    fn new(data: &'a [T]) -> Self {
        Slice {
            data,
            _phantom: std::marker::PhantomData,
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

// ============================================================================
// COMBINING CONCEPTS - REAL WORLD EXAMPLES
// ============================================================================

/// Generic cache with trait bounds
trait Cache<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn set(&mut self, key: K, value: V);
    fn remove(&mut self, key: &K) -> Option<V>;
}

struct SimpleCache<K, V>
where
    K: Eq + std::hash::Hash,
{
    data: std::collections::HashMap<K, V>,
}

impl<K, V> SimpleCache<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn new() -> Self {
        SimpleCache {
            data: std::collections::HashMap::new(),
        }
    }
}

impl<K, V> Cache<K, V> for SimpleCache<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    fn set(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
}

/// Repository pattern with generics and trait objects
trait Repository<T> {
    fn find_by_id(&self, id: u32) -> Option<T>;
    fn save(&mut self, item: T) -> u32;
    fn delete(&mut self, id: u32) -> bool;
}

struct InMemoryRepository<T> {
    items: std::collections::HashMap<u32, T>,
    next_id: u32,
}

impl<T: Clone> InMemoryRepository<T> {
    fn new() -> Self {
        InMemoryRepository {
            items: std::collections::HashMap::new(),
            next_id: 1,
        }
    }
}

impl<T: Clone> Repository<T> for InMemoryRepository<T> {
    fn find_by_id(&self, id: u32) -> Option<T> {
        self.items.get(&id).cloned()
    }

    fn save(&mut self, item: T) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.items.insert(id, item);
        id
    }

    fn delete(&mut self, id: u32) -> bool {
        self.items.remove(&id).is_some()
    }
}

/// Generic data processor with multiple trait bounds
trait Processor {
    type Input;
    type Output;
    type Error;

    fn process(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}

struct StringProcessor;

impl Processor for StringProcessor {
    type Input = String;
    type Output = Vec<String>;
    type Error = String;

    fn process(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        if input.is_empty() {
            Err("Empty input".to_string())
        } else {
            Ok(input.split_whitespace().map(String::from).collect())
        }
    }
}

/// Trait for operations with default implementations
trait Mathematic {
    fn add(&self, other: &Self) -> Self;
    fn subtract(&self, other: &Self) -> Self;

    // Default implementation
    fn multiply(&self, times: usize) -> Self
    where
        Self: Sized + Clone,
    {
        let mut result = self.clone();
        for _ in 1..times {
            result = result.add(self);
        }
        result
    }
}

#[derive(Clone)]
struct Number(i32);

impl Mathematic for Number {
    fn add(&self, other: &Self) -> Self {
        Number(self.0 + other.0)
    }

    fn subtract(&self, other: &Self) -> Self {
        Number(self.0 - other.0)
    }

    // Can override default implementation
    fn multiply(&self, times: usize) -> Self {
        Number(self.0 * times as i32)
    }
}

// ============================================================================
// MAIN FUNCTION
// ============================================================================

fn main() {
    println!("=== Traits and Generics Examples ===\n");

    // 1. Generic Functions and Structs
    println!("--- Generic Functions and Structs ---");
    print_value(42);
    print_value("Hello");
    print_pair(10, "items");

    let container = Container::new(100);
    println!("Container value: {}", container.get());

    let pair = Pair::new("first", 2);
    let (a, b) = pair.into_tuple();
    println!("Tuple: ({}, {})", a, b);

    // 2. Trait Bounds
    println!("\n--- Trait Bounds ---");
    compare_and_display(10, 20);
    compare_and_display("apple", "banana");

    let comp = Comparator::new(5);
    println!("5 > 3? {}", comp.is_greater_than(&3));

    // 3. Where Clauses
    println!("\n--- Where Clauses ---");
    complex_function("test", 42, "string");

    let result = longest_with_announcement("long", "short", "Comparing strings");
    println!("Longest: {}", result);

    // 4. Associated Types
    println!("\n--- Associated Types ---");
    let mut counter = Counter { count: 0, max: 5 };
    while let Some(value) = counter.next() {
        println!("Count: {}", value);
    }

    // 5. Generic Traits
    println!("\n--- Generic Traits ---");
    let wrapper = IntWrapper(42);
    let as_string: String = wrapper.convert();
    let as_float: f64 = wrapper.convert();
    println!("As string: {}, As float: {}", as_string, as_float);

    // 6. Trait Objects
    println!("\n--- Trait Objects ---");
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle {
            width: 4.0,
            height: 5.0,
        }),
        Box::new(Triangle {
            base: 3.0,
            height: 4.0,
        }),
    ];
    draw_shapes(&shapes);

    let shape = create_shape("circle");
    shape.draw();

    // 7. Static vs Dynamic Dispatch
    println!("\n--- Static vs Dynamic Dispatch ---");
    demonstrate_dispatch();

    // 8. Impl Trait
    println!("\n--- Impl Trait ---");
    let adder = make_adder(10);
    println!("10 + 5 = {}", adder(5));

    print_iter(1..4);

    let iter = create_iterator(1, 4);
    for item in iter {
        println!("Iterator item: {}", item);
    }

    // 9. Trait Inheritance
    println!("\n--- Trait Inheritance ---");
    let dog = Dog {
        name: "Buddy".to_string(),
        owner: "Alice".to_string(),
    };
    pet_info(&dog);
    println!("Has fur? {}", dog.has_fur());

    // 10. Blanket Implementations
    println!("\n--- Blanket Implementations ---");
    let number = 42;
    number.print();
    "Hello".print();

    // 11. Marker Traits
    println!("\n--- Marker Traits ---");
    let safe_data = SafeData { value: 100 };
    let handle = process_in_thread(safe_data);
    let _ = handle.join();

    let value = 42;
    let (v1, v2) = demonstrate_copy(value);
    println!("Copied values: {} and {}", v1, v2);

    print_maybe_sized("This works with str!");

    // 12. Advanced Patterns
    println!("\n--- Advanced Patterns ---");

    // Newtype pattern
    let distance = Meters(5000.0);
    let km = distance.to_kilometers();
    println!("Distance: {} meters = {} km", distance.0, km.0);

    let username = Username("rustacean".to_string());
    println!("Username: {}", username);

    // Type state pattern
    let door = Door::<Locked>::new();
    let door = door.unlock();
    door.open();
    let door = door.lock();

    // Builder with type state
    let person = PersonBuilder::new()
        .name("Alice".to_string())
        .age(30)
        .build();
    println!("Person: {} is {} years old", person.name, person.age);

    // Real world examples
    println!("\n--- Real World Examples ---");

    // Cache
    let mut cache = SimpleCache::new();
    cache.set("key1", "value1");
    if let Some(value) = cache.get(&"key1") {
        println!("Cache hit: {}", value);
    }

    // Repository
    let mut repo = InMemoryRepository::new();
    let id = repo.save("Item 1");
    if let Some(item) = repo.find_by_id(id) {
        println!("Found item: {}", item);
    }

    // Processor
    let processor = StringProcessor;
    match processor.process("hello world rust".to_string()) {
        Ok(words) => println!("Processed words: {:?}", words),
        Err(e) => println!("Error: {}", e),
    }

    // Mathematic trait
    let num1 = Number(5);
    let num2 = Number(3);
    let sum = num1.add(&num2);
    let product = num1.multiply(4);
    println!("Sum: {}, Product: {}", sum.0, product.0);

    println!("\n=== All examples completed successfully! ===");
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_container() {
        let container = Container::new(42);
        assert_eq!(*container.get(), 42);

        let container_str = Container::new("test");
        assert_eq!(*container_str.get(), "test");
    }

    #[test]
    fn test_comparator() {
        let comp = Comparator::new(10);
        assert!(comp.is_greater_than(&5));
        assert!(!comp.is_greater_than(&15));
    }

    #[test]
    fn test_counter_iterator() {
        let mut counter = Counter { count: 0, max: 3 };
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_converter() {
        let wrapper = IntWrapper(42);
        let as_string: String = wrapper.convert();
        let as_float: f64 = wrapper.convert();

        assert_eq!(as_string, "42");
        assert_eq!(as_float, 42.0);
    }

    #[test]
    fn test_drawable_circle() {
        let circle = Circle { radius: 2.0 };
        let area = circle.area();
        assert!((area - (std::f64::consts::PI * 4.0)).abs() < 0.001);
    }

    #[test]
    fn test_drawable_rectangle() {
        let rect = Rectangle {
            width: 4.0,
            height: 5.0,
        };
        assert_eq!(rect.area(), 20.0);
    }

    #[test]
    fn test_impl_trait() {
        let adder = make_adder(5);
        assert_eq!(adder(3), 8);
        assert_eq!(adder(10), 15);
    }

    #[test]
    fn test_dog_traits() {
        let dog = Dog {
            name: "Buddy".to_string(),
            owner: "Alice".to_string(),
        };

        assert_eq!(dog.name(), "Buddy");
        assert_eq!(dog.owner(), "Alice");
        assert!(dog.has_fur());
    }

    #[test]
    fn test_meters_kilometers() {
        let meters = Meters(5000.0);
        let km = meters.to_kilometers();
        assert_eq!(km.0, 5.0);

        let km2 = Kilometers(3.0);
        let m2 = km2.to_meters();
        assert_eq!(m2.0, 3000.0);
    }

    #[test]
    fn test_cache() {
        let mut cache = SimpleCache::new();
        cache.set("key1", 100);
        cache.set("key2", 200);

        assert_eq!(cache.get(&"key1"), Some(&100));
        assert_eq!(cache.get(&"key2"), Some(&200));
        assert_eq!(cache.get(&"key3"), None);

        assert_eq!(cache.remove(&"key1"), Some(100));
        assert_eq!(cache.get(&"key1"), None);
    }

    #[test]
    fn test_repository() {
        let mut repo = InMemoryRepository::new();

        let id1 = repo.save("Item 1".to_string());
        let id2 = repo.save("Item 2".to_string());

        assert_eq!(repo.find_by_id(id1), Some("Item 1".to_string()));
        assert_eq!(repo.find_by_id(id2), Some("Item 2".to_string()));

        assert!(repo.delete(id1));
        assert_eq!(repo.find_by_id(id1), None);
    }

    #[test]
    fn test_string_processor() {
        let processor = StringProcessor;

        match processor.process("hello world".to_string()) {
            Ok(words) => {
                assert_eq!(words.len(), 2);
                assert_eq!(words[0], "hello");
                assert_eq!(words[1], "world");
            }
            Err(_) => panic!("Should not error"),
        }

        match processor.process("".to_string()) {
            Ok(_) => panic!("Should error on empty input"),
            Err(e) => assert_eq!(e, "Empty input"),
        }
    }

    #[test]
    fn test_mathematic_trait() {
        let num1 = Number(5);
        let num2 = Number(3);

        let sum = num1.add(&num2);
        assert_eq!(sum.0, 8);

        let diff = num1.subtract(&num2);
        assert_eq!(diff.0, 2);

        let product = num1.multiply(4);
        assert_eq!(product.0, 20);
    }

    #[test]
    fn test_person_builder() {
        let person = PersonBuilder::new().name("Bob".to_string()).age(25).build();

        assert_eq!(person.name, "Bob");
        assert_eq!(person.age, 25);
    }

    #[test]
    fn test_slice_phantom() {
        let data = vec![1, 2, 3, 4, 5];
        let slice = Slice::new(&data);
        assert_eq!(slice.len(), 5);
    }
}
