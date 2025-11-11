// Structs in Rust
//
// Structs are custom data types that let you name and package together
// multiple related values. They're similar to classes in other languages,
// but without inheritance. Methods are defined in impl blocks.

fn main() {
    println!("=== Rust Structs Examples ===\n");

    // Example 1: Classic struct definition
    println!("1. Classic Structs:");
    classic_structs();
    println!();

    // Example 2: Tuple structs
    println!("2. Tuple Structs:");
    tuple_structs();
    println!();

    // Example 3: Unit structs
    println!("3. Unit Structs:");
    unit_structs();
    println!();

    // Example 4: Methods
    println!("4. Methods:");
    methods_example();
    println!();

    // Example 5: Associated functions
    println!("5. Associated Functions:");
    associated_functions();
    println!();

    // Example 6: Multiple impl blocks
    println!("6. Multiple impl Blocks:");
    multiple_impl_blocks();
    println!();

    // Example 7: Struct update syntax
    println!("7. Struct Update Syntax:");
    struct_update_syntax();
    println!();

    // Example 8: Ownership and borrowing with structs
    println!("8. Ownership and Borrowing:");
    struct_ownership();
    println!();
}

// Example 1: Classic struct
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn classic_structs() {
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1,
    };

    println!("  User: {}", user1.username);
    println!("  Email: {}", user1.email);
    println!("  Active: {}", user1.active);

    // Mutable struct
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser456"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("newemail@example.com");
    println!("  Updated email: {}", user2.email);

    // Using debug print
    println!("  Debug: {:?}", user2);
}

// Example 2: Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("  Black color: ({}, {}, {})", black.0, black.1, black.2);
    println!("  Origin point: ({}, {}, {})", origin.0, origin.1, origin.2);

    // Tuple structs are different types even with same fields
    // let color: Color = origin; // This would be a compile error!

    // Destructuring tuple structs
    let Color(r, g, b) = black;
    println!("  Destructured RGB: ({}, {}, {})", r, g, b);
}

// Example 3: Unit structs
struct AlwaysEqual;

fn unit_structs() {
    let subject = AlwaysEqual;
    // Unit structs are useful for implementing traits without storing data
    println!("  Created a unit struct instance");

    // They're often used as markers or for implementing traits
    struct Marker;
    let _marker = Marker;
    println!("  Unit structs take up zero bytes!");
}

// Example 4: Methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method with &self - borrows the instance immutably
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method that borrows mutably
    fn expand(&mut self, amount: u32) {
        self.width += amount;
        self.height += amount;
    }

    // Method that takes ownership
    fn destroy(self) -> String {
        format!(
            "Destroyed rectangle {}x{}",
            self.width, self.height
        )
    }

    // Method with additional parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn methods_example() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("  Rectangle: {:?}", rect1);
    println!("  Area: {} square pixels", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("  Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // Mutable method
    let mut rect3 = Rectangle {
        width: 20,
        height: 30,
    };
    println!("  Before expand: {:?}", rect3);
    rect3.expand(10);
    println!("  After expand: {:?}", rect3);

    // Taking ownership
    let message = rect3.destroy();
    println!("  {}", message);
    // rect3 is no longer valid here
}

// Example 5: Associated functions (like static methods)
struct Circle {
    radius: f64,
}

impl Circle {
    // Associated function - no self parameter
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    // Another constructor
    fn unit() -> Circle {
        Circle { radius: 1.0 }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn associated_functions() {
    // Call associated functions with ::
    let circle1 = Circle::new(5.0);
    println!("  Circle with radius {}: area = {:.2}", circle1.radius, circle1.area());

    let unit_circle = Circle::unit();
    println!("  Unit circle area: {:.2}", unit_circle.area());
}

// Example 6: Multiple impl blocks
struct Temperature {
    celsius: f64,
}

impl Temperature {
    fn new(celsius: f64) -> Self {
        Temperature { celsius }
    }

    fn celsius(&self) -> f64 {
        self.celsius
    }
}

impl Temperature {
    fn fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }

    fn kelvin(&self) -> f64 {
        self.celsius + 273.15
    }
}

fn multiple_impl_blocks() {
    let temp = Temperature::new(25.0);
    println!("  Temperature:");
    println!("    Celsius: {:.2}°C", temp.celsius());
    println!("    Fahrenheit: {:.2}°F", temp.fahrenheit());
    println!("    Kelvin: {:.2}K", temp.kelvin());
}

// Example 7: Struct update syntax
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

fn struct_update_syntax() {
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        city: String::from("New York"),
    };

    // Create new struct using most fields from person1
    let person2 = Person {
        name: String::from("Bob"),
        ..person1 // This moves non-Copy fields from person1
    };

    println!("  Person 2: {} is {} years old from {}",
             person2.name, person2.age, person2.city);

    // Note: person1.city has been moved to person2
    // println!("{}", person1.city); // This would be an error

    // But we can still access Copy fields
    println!("  Person 1's age (Copy): {}", person1.age);
}

// Example 8: Ownership with structs
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn new(title: &str, author: &str, pages: u32) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            pages,
        }
    }

    // Borrows self immutably
    fn description(&self) -> String {
        format!("'{}' by {} ({} pages)", self.title, self.author, self.pages)
    }

    // Returns a reference to internal data
    fn title(&self) -> &str {
        &self.title
    }

    // Mutably borrows self
    fn add_pages(&mut self, additional: u32) {
        self.pages += additional;
    }
}

fn struct_ownership() {
    let book1 = Book::new("The Rust Programming Language", "Steve Klabnik", 500);
    println!("  {}", book1.description());

    // Borrowing for method calls
    println!("  Title: {}", book1.title());

    // Mutable borrow
    let mut book2 = Book::new("Programming Rust", "Jim Blandy", 600);
    println!("  Before: {} pages", book2.pages);
    book2.add_pages(50);
    println!("  After: {} pages", book2.pages);

    // Moving ownership
    let book3 = book1; // book1 is moved to book3
    println!("  Moved book: {}", book3.description());
    // println!("{:?}", book1); // Error: book1 has been moved
}
