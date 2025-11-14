# Slint GUI

Slint is a declarative GUI toolkit for embedded and desktop applications.

## What is Slint?

Slint uses a declarative markup language (`.slint` files) for UI definition:
- **Declarative**: Define what the UI looks like, not how to build it
- **Reactive**: Automatic UI updates when data changes
- **Performant**: Compiles to native code or WASM
- **Designer-friendly**: Visual editor available

## Project Structure

```
slint-example/
├── ui/
│   └── appwindow.slint  # UI definition
├── src/
│   └── main.rs          # Rust logic
└── build.rs             # Build script
```

## Slint Language

### Basic Component

```slint
component MyButton inherits Button {
    text: "Click me";
    clicked => { debug("Clicked!"); }
}
```

### Properties and Bindings

```slint
export component Counter {
    in-out property <int> value: 0;

    Text {
        text: "Count: " + value;
    }

    Button {
        text: "Increment";
        clicked => { value += 1; }
    }
}
```

### Layouts

```slint
VerticalBox {
    HorizontalBox {
        Button { text: "1"; }
        Button { text: "2"; }
    }
    Button { text: "3"; }
}
```

## Rust Integration

### build.rs

```rust
fn main() {
    slint_build::compile("ui/appwindow.slint").unwrap();
}
```

### main.rs

```rust
slint::include_modules!();

fn main() {
    let ui = AppWindow::new().unwrap();

    // Set properties
    ui.set_counter(42);

    // Handle callbacks
    ui.on_button_clicked(|| {
        println!("Button clicked!");
    });

    ui.run().unwrap();
}
```

## Callbacks

### In Slint

```slint
export component MyApp {
    callback button-clicked();

    Button {
        clicked => { root.button-clicked(); }
    }
}
```

### In Rust

```rust
ui.on_button_clicked(|| {
    println!("Button was clicked!");
});
```

## Two-Way Bindings

```slint
export component MyApp {
    in-out property <string> text: "";

    LineEdit {
        text <=> root.text;
    }
}
```

```rust
// Get value
let text = ui.get_text();

// Set value
ui.set_text("New text".into());
```

## Running

```bash
cargo run
```

## Features

- **Cross-platform**: Desktop (Windows, macOS, Linux) and embedded
- **Small footprint**: Suitable for embedded devices
- **Hot reload**: Live preview during development
- **Accessibility**: Built-in accessibility support
- **Animations**: Declarative animations
- **Theming**: Customizable styles

## Slint Designer

Slint provides a visual designer (SlintPad):
- Live preview
- Drag-and-drop UI building
- Property editing
- Code generation

## Conditional Rendering

```slint
if counter > 0: Text {
    text: "Positive: " + counter;
    color: green;
}
```

## Loops

```slint
for item in items: Rectangle {
    Text { text: item.name; }
}
```

## Use Cases

- **Embedded GUIs**: IoT devices, industrial controls
- **Desktop Apps**: Native applications
- **Kiosks**: Touch-screen interfaces
- **Automotive**: In-vehicle infotainment
- **WASM**: Web applications

## Comparison

| Feature | Slint | egui | iced |
|---------|-------|------|------|
| Mode | Declarative | Immediate | Declarative |
| Designer | Yes | No | No |
| Embedded | Excellent | Good | Limited |
| Learning Curve | Medium | Low | Medium |

## References

- [Slint Documentation](https://slint.dev/docs)
- [Slint Language Reference](https://slint.dev/docs/slint)
- [Slint Examples](https://github.com/slint-ui/slint/tree/master/examples)
- [SlintPad (Online Editor)](https://slint.dev/editor)
