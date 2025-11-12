#![allow(dead_code)]
#![allow(unused_variables)]

use std::marker::PhantomData;
use std::mem;

// =============================================================================
// 1. TYPE ALIASES
// =============================================================================
// Type aliases create a new name for an existing type, improving readability
// and reducing repetition.

type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn type_alias_example() {
    println!("\n--- Type Aliases ---");
    let distance: Kilometers = 42;
    let another_distance: i32 = 100;

    // Type aliases are transparent - these are the same type
    let sum = distance + another_distance;
    println!("Total distance: {} km", sum);

    // Complex type alias for function pointer
    type ComplexCallback = fn(i32, String) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

// =============================================================================
// 2. NEWTYPE PATTERN
// =============================================================================
// The newtype pattern uses a tuple struct with a single field to create
// a distinct type that wraps an existing type. This provides type safety
// and enables implementing traits on external types.

#[derive(Debug, Clone, Copy, PartialEq)]
struct Meters(f64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Seconds(f64);

impl Meters {
    fn new(value: f64) -> Self {
        Meters(value)
    }

    fn value(&self) -> f64 {
        self.0
    }
}

impl Seconds {
    fn new(value: f64) -> Self {
        Seconds(value)
    }

    fn value(&self) -> f64 {
        self.0
    }
}

// Type safety: Can't accidentally mix meters and seconds
fn calculate_speed(distance: Meters, time: Seconds) -> f64 {
    distance.value() / time.value()
}

// Newtype pattern for implementing external traits on external types
struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn newtype_pattern_example() {
    println!("\n--- Newtype Pattern ---");
    let distance = Meters::new(100.0);
    let time = Seconds::new(10.0);
    let speed = calculate_speed(distance, time);
    println!("Speed: {} m/s", speed);

    // This would cause a compile error (type mismatch):
    // calculate_speed(time, distance);

    let wrapper = Wrapper(vec!["hello".to_string(), "world".to_string()]);
    println!("Wrapper display: {}", wrapper);
}

// =============================================================================
// 3. DYNAMICALLY SIZED TYPES (DSTs)
// =============================================================================
// DSTs are types whose size is not known at compile time. They can only be
// used behind a pointer (reference, Box, Rc, Arc, etc.).

fn dst_example() {
    println!("\n--- Dynamically Sized Types ---");

    // str is a DST - we can only use it as &str, Box<str>, etc.
    let s1: &str = "Hello, DST!";
    println!("String slice: {}", s1);

    // [T] is a DST - we can only use it as &[T], Box<[T]>, etc.
    let slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("Slice: {:?}", slice);

    // Trait objects are DSTs
    let trait_object: &dyn std::fmt::Debug = &42;
    println!("Trait object: {:?}", trait_object);

    // The ?Sized trait bound allows generic types to be DSTs
    fn print_debug<T: std::fmt::Debug + ?Sized>(value: &T) {
        println!("Debug: {:?}", value);
    }

    print_debug("This is a str slice"); // str is a DST
    print_debug(&[1, 2, 3]); // [i32] is a DST
}

// =============================================================================
// 4. NEVER TYPE (!)
// =============================================================================
// The never type (!) represents computations that never return. It's useful
// for functions that panic, loop forever, or exit the process.

fn never_type_example() {
    println!("\n--- Never Type ---");

    // Function that never returns
    fn diverges() -> ! {
        panic!("This function never returns normally!");
    }

    // Never type can coerce to any type
    #[allow(clippy::manual_unwrap_or_default)]
    #[allow(clippy::manual_unwrap_or)]
    let result: i32 = match "123".parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            // Could use: diverges() here, which returns !
            // The ! type coerces to i32
            0 // Using 0 instead to avoid panic
        }
    };

    println!("Parsed result: {}", result);

    // Loop with never type
    #[allow(clippy::empty_loop)]
    fn infinite_loop() -> ! {
        loop {
            // This loop never exits
        }
    }

    // continue and break also have type !
    let mut counter = 0;
    let value = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break with value has type !
        }
    };
    println!("Loop value: {}", value);
}

// =============================================================================
// 5. FUNCTION POINTERS
// =============================================================================
// Function pointers allow storing and passing functions as values.
// Unlike closures, function pointers are always Fn (not FnMut or FnOnce).

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Function that returns a function pointer
fn get_function(add: bool) -> fn(i32) -> i32 {
    fn increment(x: i32) -> i32 {
        x + 1
    }
    fn decrement(x: i32) -> i32 {
        x - 1
    }

    if add { increment } else { decrement }
}

fn function_pointer_example() {
    println!("\n--- Function Pointers ---");

    let result = do_twice(add_one, 5);
    println!("do_twice(add_one, 5) = {}", result);

    // Store function pointer in a variable
    let func: fn(i32) -> i32 = add_one;
    println!("func(10) = {}", func(10));

    // Get function pointer dynamically
    let operation = get_function(true);
    println!("operation(20) = {}", operation(20));

    // Function pointers implement all three closure traits
    let numbers = [1, 2, 3];
    let strings: Vec<String> = numbers.iter().map(ToString::to_string).collect();
    println!("Strings: {:?}", strings);
}

// =============================================================================
// 6. TRAIT OBJECTS AND SIZING
// =============================================================================
// Trait objects are DSTs that use fat pointers (pointer + vtable).

trait Animal {
    fn make_sound(&self) -> &str;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) -> &str {
        "Woof!"
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> &str {
        "Meow!"
    }
}

fn trait_object_example() {
    println!("\n--- Trait Objects and Sizing ---");

    let dog = Dog;
    let cat = Cat;

    // Trait objects must be behind a pointer
    let animals: Vec<&dyn Animal> = vec![&dog, &cat];

    for animal in &animals {
        println!("Animal says: {}", animal.make_sound());
    }

    // Size of concrete types vs trait objects
    println!("Size of Dog: {} bytes", mem::size_of::<Dog>());
    println!("Size of Cat: {} bytes", mem::size_of::<Cat>());
    println!(
        "Size of &dyn Animal: {} bytes",
        mem::size_of::<&dyn Animal>()
    );
    println!(
        "Size of Box<dyn Animal>: {} bytes",
        mem::size_of::<Box<dyn Animal>>()
    );

    // &dyn Trait is twice the size of a normal reference (fat pointer)
    println!("Size of &Dog: {} bytes", mem::size_of::<&Dog>());
}

// =============================================================================
// 7. ZERO-SIZED TYPES (ZSTs)
// =============================================================================
// ZSTs are types that occupy zero bytes of memory. They're useful for
// type-level programming and state machines.

struct ZeroSized; // Unit struct is a ZST

struct AnotherZst;

impl AnotherZst {
    fn new() -> Self {
        AnotherZst
    }

    fn do_something(&self) {
        println!("Doing something with zero memory overhead!");
    }
}

// ZST for state machine
struct Locked;
struct Unlocked;

struct StateMachine<State> {
    _state: PhantomData<State>,
}

impl StateMachine<Locked> {
    fn new() -> Self {
        println!("Creating locked state machine");
        StateMachine {
            _state: PhantomData,
        }
    }

    fn unlock(self) -> StateMachine<Unlocked> {
        println!("Unlocking state machine");
        StateMachine {
            _state: PhantomData,
        }
    }
}

impl StateMachine<Unlocked> {
    fn lock(self) -> StateMachine<Locked> {
        println!("Locking state machine");
        StateMachine {
            _state: PhantomData,
        }
    }

    fn access_data(&self) {
        println!("Accessing data (only available when unlocked)");
    }
}

fn zero_sized_types_example() {
    println!("\n--- Zero-Sized Types ---");

    println!("Size of ZeroSized: {} bytes", mem::size_of::<ZeroSized>());
    println!("Size of AnotherZst: {} bytes", mem::size_of::<AnotherZst>());
    println!("Size of (): {} bytes", mem::size_of::<()>());

    let zst = AnotherZst::new();
    zst.do_something();

    // State machine with ZSTs
    let machine = StateMachine::<Locked>::new();
    let machine = machine.unlock();
    machine.access_data();
    // machine.access_data() only compiles in Unlocked state
    let machine = machine.lock();
    // machine.access_data(); // This would be a compile error!

    println!(
        "Size of StateMachine<Locked>: {} bytes",
        mem::size_of::<StateMachine<Locked>>()
    );
}

// =============================================================================
// 8. PHANTOM DATA
// =============================================================================
// PhantomData is used to mark that a type logically contains data of type T
// even though it doesn't physically store it. This is crucial for variance,
// drop check, and type-level programming.

struct PhantomExample<'a, T> {
    // Without PhantomData, this struct wouldn't be tied to 'a or T
    data: *const T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> PhantomExample<'a, T> {
    fn new(reference: &'a T) -> Self {
        PhantomExample {
            data: reference as *const T,
            _marker: PhantomData,
        }
    }

    unsafe fn get(&self) -> &'a T {
        unsafe { &*self.data }
    }
}

// PhantomData for preventing auto-trait implementation
struct NotSendOrSync {
    data: i32,
    _marker: PhantomData<*const ()>, // *const () is not Send or Sync
}

// PhantomData for type-level branding
struct Branded<T, Brand> {
    value: T,
    _brand: PhantomData<Brand>,
}

struct UserId;
struct ProductId;

impl<T, Brand> Branded<T, Brand> {
    fn new(value: T) -> Self {
        Branded {
            value,
            _brand: PhantomData,
        }
    }

    fn value(&self) -> &T {
        &self.value
    }
}

fn phantom_data_example() {
    println!("\n--- PhantomData ---");

    let x = 42;
    let phantom = PhantomExample::new(&x);
    unsafe {
        println!("PhantomData value: {}", phantom.get());
    }

    // Type branding example
    let user_id = Branded::<i32, UserId>::new(12345);
    let product_id = Branded::<i32, ProductId>::new(67890);

    println!("User ID: {}", user_id.value());
    println!("Product ID: {}", product_id.value());

    // These are different types even though they wrap the same underlying type
    // This would be a compile error:
    // let same: Branded<i32, UserId> = product_id;

    println!(
        "Size of PhantomData<T>: {} bytes",
        mem::size_of::<PhantomData<i32>>()
    );
}

// =============================================================================
// 9. TYPE COERCION
// =============================================================================
// Rust performs automatic type coercion in certain situations.

fn type_coercion_example() {
    println!("\n--- Type Coercion ---");

    // Array to slice coercion
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &array; // Coercion happens here
    println!("Slice from array: {:?}", slice);

    // Concrete type to trait object coercion
    let dog = Dog;
    let animal: &dyn Animal = &dog; // Coercion to trait object
    println!("Coerced animal: {}", animal.make_sound());

    // Mutable reference to immutable reference
    let mut x = 42;
    let y: &i32 = &mut x; // &mut T coerces to &T
    println!("Coerced reference: {}", y);

    // Lifetime coercion (subtyping)
    fn takes_short(_: &str) {}
    let long_string = String::from("long lived");
    takes_short(&long_string); // Longer lifetime coerces to shorter
}

// =============================================================================
// 10. OPAQUE TYPES (impl Trait)
// =============================================================================
// impl Trait allows returning types that implement a trait without
// specifying the concrete type.

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn returns_iterator() -> impl Iterator<Item = i32> {
    (0..5).map(|x| x * 2)
}

// impl Trait in argument position
fn takes_impl_trait(x: impl std::fmt::Display) {
    println!("Display: {}", x);
}

fn opaque_types_example() {
    println!("\n--- Opaque Types (impl Trait) ---");

    let closure = returns_closure();
    println!("Closure result: {}", closure(10));

    let iter = returns_iterator();
    let collected: Vec<i32> = iter.collect();
    println!("Iterator results: {:?}", collected);

    takes_impl_trait(42);
    takes_impl_trait("Hello");
}

// =============================================================================
// 11. ASSOCIATED TYPES VS GENERIC PARAMETERS
// =============================================================================

// Using associated types - cleaner for single implementation
trait Container {
    type Item;
    fn get(&self) -> &Self::Item;
}

struct IntContainer(i32);

impl Container for IntContainer {
    type Item = i32;

    fn get(&self) -> &Self::Item {
        &self.0
    }
}

// Using generic parameters - allows multiple implementations
trait GenericContainer<T> {
    fn get(&self) -> &T;
}

struct MultiContainer(i32, String);

impl GenericContainer<i32> for MultiContainer {
    fn get(&self) -> &i32 {
        &self.0
    }
}

impl GenericContainer<String> for MultiContainer {
    fn get(&self) -> &String {
        &self.1
    }
}

fn associated_types_example() {
    println!("\n--- Associated Types vs Generic Parameters ---");

    let int_container = IntContainer(42);
    println!("Container item: {}", int_container.get());

    let multi = MultiContainer(100, "Hello".to_string());
    let int_val: &i32 = GenericContainer::get(&multi);
    let str_val: &String = GenericContainer::get(&multi);
    println!("Multi container: {} and {}", int_val, str_val);
}

// =============================================================================
// 12. SIZED TRAIT AND ?SIZED
// =============================================================================

// Generic function requiring Sized (default)
fn sized_function<T>(value: T) {
    println!("Size of T: {} bytes", mem::size_of::<T>());
}

// Generic function allowing unsized types
fn unsized_function<T: ?Sized>(value: &T) {
    println!("Working with potentially unsized type");
}

fn sized_trait_example() {
    println!("\n--- Sized Trait and ?Sized ---");

    sized_function(42);
    sized_function("Hello"); // &str is Sized, but str is not

    // Works with both sized and unsized types
    unsized_function(&42);
    unsized_function("Hello" as &str); // str is unsized

    println!("Is i32 Sized? {}", std::mem::size_of::<i32>() > 0);
    println!("Size of &str: {} bytes", mem::size_of::<&str>());
}

// =============================================================================
// MAIN FUNCTION
// =============================================================================

fn main() {
    println!("=== Advanced Types ===");

    type_alias_example();
    newtype_pattern_example();
    dst_example();
    never_type_example();
    function_pointer_example();
    trait_object_example();
    zero_sized_types_example();
    phantom_data_example();
    type_coercion_example();
    opaque_types_example();
    associated_types_example();
    sized_trait_example();

    println!("\n=== All examples completed successfully! ===");
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newtype_pattern() {
        let distance = Meters::new(100.0);
        let time = Seconds::new(10.0);
        let speed = calculate_speed(distance, time);
        assert_eq!(speed, 10.0);

        // Test equality
        assert_eq!(Meters::new(5.0), Meters::new(5.0));
        assert_ne!(Meters::new(5.0), Meters::new(10.0));
    }

    #[test]
    fn test_function_pointers() {
        let result = do_twice(add_one, 5);
        assert_eq!(result, 12); // (5+1) + (5+1) = 12

        let increment = get_function(true);
        assert_eq!(increment(10), 11);

        let decrement = get_function(false);
        assert_eq!(decrement(10), 9);
    }

    #[test]
    fn test_zero_sized_types() {
        // ZSTs have zero size
        assert_eq!(mem::size_of::<ZeroSized>(), 0);
        assert_eq!(mem::size_of::<AnotherZst>(), 0);
        assert_eq!(mem::size_of::<()>(), 0);

        // PhantomData is also a ZST
        assert_eq!(mem::size_of::<PhantomData<i32>>(), 0);
        assert_eq!(mem::size_of::<PhantomData<String>>(), 0);

        // State machines using ZSTs have zero overhead
        assert_eq!(mem::size_of::<StateMachine<Locked>>(), 0);
        assert_eq!(mem::size_of::<StateMachine<Unlocked>>(), 0);
    }

    #[test]
    fn test_trait_objects() {
        let dog = Dog;
        let cat = Cat;

        let animals: Vec<&dyn Animal> = vec![&dog, &cat];

        assert_eq!(animals[0].make_sound(), "Woof!");
        assert_eq!(animals[1].make_sound(), "Meow!");

        // Trait object references are fat pointers (2x pointer size)
        assert_eq!(mem::size_of::<&dyn Animal>(), 2 * mem::size_of::<&Dog>());
    }

    #[test]
    fn test_branded_types() {
        let user_id = Branded::<i32, UserId>::new(12345);
        let product_id = Branded::<i32, ProductId>::new(67890);

        assert_eq!(*user_id.value(), 12345);
        assert_eq!(*product_id.value(), 67890);

        // Branded types have no runtime overhead
        assert_eq!(mem::size_of_val(&user_id), mem::size_of::<i32>());
    }

    #[test]
    fn test_opaque_types() {
        let closure = returns_closure();
        assert_eq!(closure(10), 11);
        assert_eq!(closure(0), 1);

        let iter = returns_iterator();
        let collected: Vec<i32> = iter.collect();
        assert_eq!(collected, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_associated_types() {
        let container = IntContainer(42);
        assert_eq!(*container.get(), 42);

        let multi = MultiContainer(100, "Hello".to_string());
        let int_val: &i32 = GenericContainer::get(&multi);
        let str_val: &String = GenericContainer::get(&multi);
        assert_eq!(*int_val, 100);
        assert_eq!(str_val, "Hello");
    }
}
