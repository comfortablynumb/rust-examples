#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::approx_constant)]

// Smart Pointers in Rust (Intermediate)
//
// Comprehensive coverage of smart pointers including:
// - Box<T>, Rc<T>, Arc<T>, RefCell<T>, Mutex<T>, RwLock<T>
// - Weak<T> for breaking cycles
// - Cow<T> for clone-on-write
// - Custom smart pointers
// - Advanced patterns and use cases

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::{Rc, Weak as RcWeak};
use std::sync::{Arc, Mutex, RwLock, Weak as ArcWeak};
use std::thread;

fn main() {
    println!("=== Smart Pointers (Intermediate) ===\n");

    // Section 1: Box<T>
    println!("1. Box<T> - Heap Allocation:");
    box_examples();
    println!();

    // Section 2: Rc<T> and Weak<T>
    println!("2. Rc<T> and Weak<T>:");
    rc_examples();
    println!();

    // Section 3: RefCell<T> and Cell<T>
    println!("3. Interior Mutability (RefCell, Cell):");
    interior_mutability();
    println!();

    // Section 4: Rc<RefCell<T>> pattern
    println!("4. Rc<RefCell<T>> - Shared Mutable State:");
    rc_refcell_pattern();
    println!();

    // Section 5: Arc<T> and Arc<Mutex<T>>
    println!("5. Arc<T> - Thread-Safe Reference Counting:");
    arc_examples();
    println!();

    // Section 6: Arc<RwLock<T>>
    println!("6. Arc<RwLock<T>> - Multiple Readers:");
    rwlock_examples();
    println!();

    // Section 7: Weak references and cycle breaking
    println!("7. Breaking Reference Cycles:");
    cycle_breaking();
    println!();

    // Section 8: Cow<T> - Clone-on-Write
    println!("8. Cow<T> - Clone-on-Write:");
    cow_examples();
    println!();

    // Section 9: Custom smart pointers
    println!("9. Custom Smart Pointers:");
    custom_smart_pointers();
    println!();

    // Section 10: Performance considerations
    println!("10. Performance Considerations:");
    performance_notes();
    println!();
}

// =============================================================================
// 1. Box<T> Examples
// =============================================================================

fn box_examples() {
    // Basic heap allocation
    let boxed_int = Box::new(5);
    println!("  Boxed int: {}", boxed_int);

    // Box for large data
    let large_array = Box::new([0; 1000]);
    println!("  Boxed large array (stack would overflow)");

    // Box for trait objects
    let shape: Box<dyn Shape> = Box::new(Circle { radius: 5.0 });
    println!("  Area of boxed trait object: {}", shape.area());

    // Box::leak for 'static lifetime
    let leaked: &'static mut i32 = Box::leak(Box::new(42));
    *leaked = 43;
    println!("  Leaked value: {}", leaked);

    // Recursive type
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("  Recursive list created");
}

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}

// Recursive type
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

// =============================================================================
// 2. Rc<T> and Weak<T> Examples
// =============================================================================

fn rc_examples() {
    // Basic Rc usage
    let shared = Rc::new(42);
    println!("  Initial ref count: {}", Rc::strong_count(&shared));

    {
        let shared2 = Rc::clone(&shared);
        let shared3 = Rc::clone(&shared);
        println!("  Ref count after clones: {}", Rc::strong_count(&shared));
        println!("  Value: {}", *shared2);
    }

    println!("  Ref count after scope: {}", Rc::strong_count(&shared));

    // Weak references
    let strong = Rc::new(String::from("Hello"));
    let weak = Rc::downgrade(&strong);

    println!(
        "  Strong: {}, Weak: {}",
        Rc::strong_count(&strong),
        Rc::weak_count(&strong)
    );

    // Upgrade weak to strong
    if let Some(upgraded) = weak.upgrade() {
        println!("  Upgraded weak reference: {}", *upgraded);
    }

    drop(strong);

    // Weak reference after strong dropped
    if weak.upgrade().is_none() {
        println!("  Weak reference is now invalid");
    }
}

// =============================================================================
// 3. Interior Mutability (RefCell, Cell)
// =============================================================================

fn interior_mutability() {
    // RefCell for runtime borrow checking
    let data = RefCell::new(5);

    println!("  Initial value: {}", data.borrow());

    // Mutable borrow
    *data.borrow_mut() += 10;
    println!("  After mutation: {}", data.borrow());

    // Multiple immutable borrows OK
    {
        let borrow1 = data.borrow();
        let borrow2 = data.borrow();
        println!("  Multiple borrows: {}, {}", *borrow1, *borrow2);
    }

    // Cell for Copy types
    let cell = Cell::new(5);
    println!("  Cell value: {}", cell.get());

    cell.set(10);
    println!("  After set: {}", cell.get());

    // replace returns old value
    let old = cell.replace(20);
    println!("  Old: {}, New: {}", old, cell.get());
}

// =============================================================================
// 4. Rc<RefCell<T>> Pattern
// =============================================================================

fn rc_refcell_pattern() {
    // Shared mutable state (single-threaded)
    let shared_state = Rc::new(RefCell::new(vec![1, 2, 3]));

    let state1 = Rc::clone(&shared_state);
    let state2 = Rc::clone(&shared_state);

    state1.borrow_mut().push(4);
    println!("  After push from state1: {:?}", *state2.borrow());

    state2.borrow_mut().push(5);
    println!("  After push from state2: {:?}", *state1.borrow());

    println!("  Final ref count: {}", Rc::strong_count(&shared_state));
}

// =============================================================================
// 5. Arc<T> Examples
// =============================================================================

fn arc_examples() {
    let shared_data = Arc::new(vec![1, 2, 3, 4, 5]);

    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            let sum: i32 = data.iter().sum();
            println!("  Thread {}: sum = {}", i, sum);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  All threads completed");
}

// =============================================================================
// 6. Arc<RwLock<T>> Examples
// =============================================================================

fn rwlock_examples() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    let mut handles = vec![];

    // Multiple readers
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let read_guard = data.read().unwrap();
            println!("  Reader {}: {:?}", i, *read_guard);
        });
        handles.push(handle);
    }

    // One writer
    {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut write_guard = data.write().unwrap();
            write_guard.push(4);
            println!("  Writer: added 4");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  Final data: {:?}", *data.read().unwrap());
}

// =============================================================================
// 7. Breaking Reference Cycles
// =============================================================================

fn cycle_breaking() {
    // Tree structure with parent and child
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(RcWeak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "  Leaf: strong={}, weak={}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(RcWeak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "  Branch: strong={}, weak={}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "  Leaf: strong={}, weak={}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("  After branch dropped:");
    println!(
        "  Leaf: strong={}, weak={}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // Parent is dropped, weak reference is invalid
    println!("  Leaf parent: {:?}", leaf.parent.borrow().upgrade());
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<RcWeak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// =============================================================================
// 8. Cow<T> Examples
// =============================================================================

fn cow_examples() {
    // Borrowed
    let text: Cow<str> = Cow::Borrowed("Hello");
    println!("  Borrowed: {}", text);

    // Owned
    let text: Cow<str> = Cow::Owned(String::from("World"));
    println!("  Owned: {}", text);

    // Clone-on-write
    let mut text: Cow<str> = Cow::Borrowed("Hello");
    println!("  Before mutation: {}", text);

    text.to_mut().push_str(" World");
    println!("  After mutation: {}", text);

    // Function accepting Cow
    fn process(s: Cow<str>) -> Cow<str> {
        if s.contains("bad") {
            Cow::Owned(s.replace("bad", "good"))
        } else {
            s
        }
    }

    let s1 = process(Cow::Borrowed("This is fine"));
    let s2 = process(Cow::Borrowed("This is bad"));
    println!("  Processed 1: {}", s1);
    println!("  Processed 2: {}", s2);

    // Cow with slice
    let data = vec![1, 2, 3];
    let mut cow_slice: Cow<[i32]> = Cow::Borrowed(&data);
    println!("  Cow slice (borrowed): {:?}", cow_slice);

    cow_slice.to_mut().push(4);
    println!("  Cow slice (owned): {:?}", cow_slice);
}

// =============================================================================
// 9. Custom Smart Pointers
// =============================================================================

fn custom_smart_pointers() {
    // Custom Box-like type
    let custom = MyBox::new(42);
    println!("  Custom smart pointer: {}", *custom);

    // Deref coercion
    let s = MyBox::new(String::from("Hello"));
    print_str(&s); // Deref coercion: &MyBox<String> -> &String -> &str

    // Custom reference counting
    let shared = SimpleRc::new(100);
    println!("  SimpleRc value: {}", *shared);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn print_str(s: &str) {
    println!("  Deref coercion worked: {}", s);
}

// Simple reference counting (educational purpose only)
struct SimpleRc<T> {
    value: *const T,
    // In real Rc, this would be reference counted
}

impl<T> SimpleRc<T> {
    fn new(value: T) -> SimpleRc<T> {
        SimpleRc {
            value: Box::into_raw(Box::new(value)),
        }
    }
}

impl<T> Deref for SimpleRc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.value }
    }
}

impl<T> Drop for SimpleRc<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.value as *mut T));
        }
    }
}

// =============================================================================
// 10. Performance Considerations
// =============================================================================

fn performance_notes() {
    println!("  Box<T>:");
    println!("    - Single allocation");
    println!("    - No overhead beyond pointer");
    println!("    - Best for single ownership");

    println!("\n  Rc<T>:");
    println!("    - Reference counting overhead");
    println!("    - Not thread-safe");
    println!("    - Use for shared ownership (single-threaded)");

    println!("\n  Arc<T>:");
    println!("    - Atomic reference counting");
    println!("    - Thread-safe but slower than Rc");
    println!("    - Use for shared ownership (multi-threaded)");

    println!("\n  RefCell<T>:");
    println!("    - Runtime borrow checking");
    println!("    - Panics on violations");
    println!("    - Use sparingly");

    println!("\n  Mutex<T>:");
    println!("    - Locking overhead");
    println!("    - Can block threads");
    println!("    - Use for mutable shared state");

    println!("\n  RwLock<T>:");
    println!("    - Multiple readers or one writer");
    println!("    - More overhead than Mutex");
    println!("    - Use when reads dominate");

    println!("\n  Cow<T>:");
    println!("    - Zero-cost if no mutation");
    println!("    - Clones only when needed");
    println!("    - Great for conditional mutations");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box() {
        let b = Box::new(5);
        assert_eq!(*b, 5);
    }

    #[test]
    fn test_rc() {
        let rc = Rc::new(42);
        let rc2 = Rc::clone(&rc);
        assert_eq!(Rc::strong_count(&rc), 2);
        assert_eq!(*rc, *rc2);
    }

    #[test]
    fn test_refcell() {
        let data = RefCell::new(5);
        *data.borrow_mut() = 10;
        assert_eq!(*data.borrow(), 10);
    }

    #[test]
    fn test_arc() {
        let arc = Arc::new(vec![1, 2, 3]);
        let arc2 = Arc::clone(&arc);
        assert_eq!(Arc::strong_count(&arc), 2);
        assert_eq!(*arc, *arc2);
    }

    #[test]
    fn test_cow_borrowed() {
        let cow: Cow<str> = Cow::Borrowed("hello");
        assert!(matches!(cow, Cow::Borrowed(_)));
    }

    #[test]
    fn test_cow_mutation() {
        let mut cow: Cow<str> = Cow::Borrowed("hello");
        cow.to_mut().push_str(" world");
        assert!(matches!(cow, Cow::Owned(_)));
        assert_eq!(cow, "hello world");
    }

    #[test]
    fn test_weak_upgrade() {
        let strong = Rc::new(42);
        let weak = Rc::downgrade(&strong);
        assert!(weak.upgrade().is_some());
        drop(strong);
        assert!(weak.upgrade().is_none());
    }
}
