#![allow(dead_code)]
#![allow(clippy::useless_vec)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::manual_map)]
#![allow(clippy::unnecessary_literal_unwrap)]
#![allow(clippy::bind_instead_of_map)]
#![allow(clippy::unnecessary_fold)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::let_and_return)]
#![allow(unused_variables)]
#![allow(clippy::iter_count)]

// Smart Pointers in Rust
//
// Smart pointers are data structures that act like pointers but have additional
// metadata and capabilities. Unlike references, smart pointers often own the data
// they point to. Common smart pointers: Box<T>, Rc<T>, Arc<T>, RefCell<T>.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    println!("=== Rust Smart Pointers Examples ===\n");

    // Example 1: Box<T> - heap allocation
    println!("1. Box<T> - Heap Allocation:");
    box_examples();
    println!();

    // Example 2: Recursive types with Box
    println!("2. Recursive Types:");
    recursive_types();
    println!();

    // Example 3: Rc<T> - reference counting
    println!("3. Rc<T> - Reference Counting:");
    rc_examples();
    println!();

    // Example 4: RefCell<T> - interior mutability
    println!("4. RefCell<T> - Interior Mutability:");
    refcell_examples();
    println!();

    // Example 5: Combining Rc<RefCell<T>>
    println!("5. Rc<RefCell<T>> - Shared Mutable State:");
    rc_refcell_examples();
    println!();

    // Example 6: Arc<T> - atomic reference counting
    println!("6. Arc<T> - Thread-Safe Reference Counting:");
    arc_examples();
    println!();

    // Example 7: Deref and Drop traits
    println!("7. Deref and Drop Traits:");
    deref_drop_examples();
    println!();
}

// Example 1: Box<T> - Store data on heap
fn box_examples() {
    // Simple heap allocation
    let boxed_int = Box::new(5);
    println!("  Boxed integer: {}", boxed_int);

    // Box allows moving large data cheaply
    let large_data = Box::new([0; 1000]);
    println!("  Boxed large array (1000 elements)");

    // Box owns the data - dropped when box goes out of scope
    {
        let temp_box = Box::new(String::from("temporary"));
        println!("  Temporary: {}", temp_box);
    } // temp_box dropped here

    // Box is useful for trait objects
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

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];

    for animal in animals {
        println!("  Animal says: {}", animal.make_sound());
    }
}

// Example 2: Recursive types with Box
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self, elem: i32) -> List {
        List::Cons(elem, Box::new(self))
    }

    fn len(&self) -> usize {
        match self {
            List::Cons(_, tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }
}

fn recursive_types() {
    let list = List::new().prepend(1).prepend(2).prepend(3);

    println!("  List length: {}", list.len());

    // Binary tree example
    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>,
    }

    let tree = TreeNode {
        value: 1,
        left: Some(Box::new(TreeNode {
            value: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            value: 3,
            left: None,
            right: None,
        })),
    };

    println!("  Tree root: {}", tree.value);
}

// Example 3: Rc<T> - Reference counted pointer (single-threaded)
fn rc_examples() {
    let shared_data = Rc::new(String::from("shared"));
    println!("  Created Rc, count: {}", Rc::strong_count(&shared_data));

    {
        let shared_data2 = Rc::clone(&shared_data);
        println!("  Cloned Rc, count: {}", Rc::strong_count(&shared_data));
        println!("  Data: {}", shared_data2);

        let shared_data3 = Rc::clone(&shared_data);
        println!("  Cloned again, count: {}", Rc::strong_count(&shared_data));
        let _ = shared_data3;
    } // shared_data2 and shared_data3 dropped here

    println!("  After scope, count: {}", Rc::strong_count(&shared_data));

    // Rc enables multiple ownership
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: Vec<Rc<Node>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        children: vec![],
    });

    let branch = Node {
        value: 5,
        children: vec![Rc::clone(&leaf)],
    };

    println!("  Leaf strong count: {}", Rc::strong_count(&leaf));
    println!("  Branch value: {}", branch.value);
}

// Example 4: RefCell<T> - Interior mutability
fn refcell_examples() {
    let data = RefCell::new(5);

    println!("  Initial value: {}", data.borrow());

    // Mutate through immutable reference
    *data.borrow_mut() += 10;
    println!("  After mutation: {}", data.borrow());

    // Multiple immutable borrows OK
    {
        let borrow1 = data.borrow();
        let borrow2 = data.borrow();
        println!("  Multiple borrows: {} and {}", borrow1, borrow2);
    }

    // Mutable borrow (exclusive)
    {
        let mut borrow_mut = data.borrow_mut();
        *borrow_mut += 5;
        println!("  Mutable borrow: {}", borrow_mut);
    }

    println!("  Final value: {}", data.borrow());

    // RefCell enables interior mutability pattern
    struct Config {
        settings: RefCell<Vec<String>>,
    }

    let config = Config {
        settings: RefCell::new(vec![]),
    };

    // Can modify through immutable reference
    config.settings.borrow_mut().push(String::from("setting1"));
    config.settings.borrow_mut().push(String::from("setting2"));

    println!("  Config settings: {:?}", config.settings.borrow());
}

// Example 5: Rc<RefCell<T>> - Shared mutable state
fn rc_refcell_examples() {
    let shared_list = Rc::new(RefCell::new(vec![1, 2, 3]));

    let list1 = Rc::clone(&shared_list);
    let list2 = Rc::clone(&shared_list);

    println!("  Initial: {:?}", shared_list.borrow());

    // Mutate through first reference
    list1.borrow_mut().push(4);
    println!("  After list1 push: {:?}", shared_list.borrow());

    // Mutate through second reference
    list2.borrow_mut().push(5);
    println!("  After list2 push: {:?}", shared_list.borrow());

    println!("  Strong count: {}", Rc::strong_count(&shared_list));

    // Practical example: Graph with shared nodes
    #[derive(Debug)]
    struct GraphNode {
        id: usize,
        neighbors: RefCell<Vec<Rc<GraphNode>>>,
    }

    let node1 = Rc::new(GraphNode {
        id: 1,
        neighbors: RefCell::new(vec![]),
    });

    let node2 = Rc::new(GraphNode {
        id: 2,
        neighbors: RefCell::new(vec![]),
    });

    // Create bidirectional connection
    node1.neighbors.borrow_mut().push(Rc::clone(&node2));
    node2.neighbors.borrow_mut().push(Rc::clone(&node1));

    println!("  Node 1 has {} neighbors", node1.neighbors.borrow().len());
    println!("  Node 2 has {} neighbors", node2.neighbors.borrow().len());
}

// Example 6: Arc<T> - Thread-safe reference counting
fn arc_examples() {
    let shared_data = Arc::new(vec![1, 2, 3, 4, 5]);

    println!("  Initial Arc count: {}", Arc::strong_count(&shared_data));

    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&shared_data);
        let handle = std::thread::spawn(move || {
            println!("    Thread {}: {:?}", i, data);
        });
        handles.push(handle);
    }

    println!(
        "  Arc count with threads: {}",
        Arc::strong_count(&shared_data)
    );

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  Final Arc count: {}", Arc::strong_count(&shared_data));

    // Arc with Mutex for shared mutable state across threads
    use std::sync::Mutex;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("  Counter result: {}", *counter.lock().unwrap());
}

// Example 7: Deref and Drop traits
fn deref_drop_examples() {
    // Custom smart pointer
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // Implement Deref
    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // Deref coercion
    println!("  MyBox deref works!");

    // Drop trait - custom cleanup
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("  Dropping CustomSmartPointer with data: {}", self.data);
        }
    }

    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        println!("  CustomSmartPointer created: {}", c.data);

        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("  Another CustomSmartPointer created: {}", d.data);
    } // c and d dropped here (d first, then c - LIFO order)

    println!("  After scope");
}
