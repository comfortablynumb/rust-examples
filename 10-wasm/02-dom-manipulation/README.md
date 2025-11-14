# DOM Manipulation with WebAssembly

Demonstrates how to interact with the browser DOM from Rust/WASM using the `web-sys` crate.

## Concepts Covered

- Using `web-sys` for browser APIs
- DOM element creation and manipulation
- Working with HTML elements
- CSS class manipulation
- Input handling
- Console logging from WASM

## Code Examples

### Accessing Browser APIs

```rust
use web_sys::{console, Document, Window};

fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> Document {
    window()
        .document()
        .expect("should have a document on window")
}
```

### Creating Elements

```rust
#[wasm_bindgen]
pub fn create_element(tag: &str, text: &str) -> Result<(), JsValue> {
    let document = document();
    let body = document.body().expect("document should have a body");

    let element = document.create_element(tag)?;
    element.set_text_content(Some(text));
    body.append_child(&element)?;

    Ok(())
}
```

### Manipulating Elements

```rust
#[wasm_bindgen]
pub fn update_element(id: &str, new_text: &str) -> Result<(), JsValue> {
    let document = document();
    let element = document.get_element_by_id(id)?;
    element.set_text_content(Some(new_text));
    Ok(())
}
```

### Working with Input

```rust
#[wasm_bindgen]
pub fn get_input_value(id: &str) -> Result<String, JsValue> {
    let document = document();
    let element = document.get_element_by_id(id)?;
    let input: HtmlInputElement = element.dyn_into()?;
    Ok(input.value())
}
```

## Building

```bash
wasm-pack build --target web
```

## Example HTML

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>DOM Manipulation</title>
    <style>
        .highlight { background-color: yellow; }
        .hidden { display: none; }
    </style>
</head>
<body>
    <div id="app"></div>

    <script type="module">
        import init, {
            log,
            create_element,
            create_button,
            create_input,
            create_list,
            get_input_value,
            update_element,
            toggle_class
        } from './pkg/dom_manipulation.js';

        async function run() {
            await init();

            log("WebAssembly loaded!");

            // Create heading
            create_element("h1", "DOM Manipulation with Rust/WASM");

            // Create input
            create_input("nameInput", "Enter your name");

            // Create button
            create_button("Click me!", "greetBtn");

            // Add click handler
            document.getElementById("greetBtn").addEventListener("click", () => {
                const name = get_input_value("nameInput");
                create_element("p", `Hello, ${name}!`);
            });

            // Create a list
            create_list(["Item 1", "Item 2", "Item 3"]);
        }

        run();
    </script>
</body>
</html>
```

## web-sys Features

The `web-sys` crate provides bindings to Web APIs. Enable only the features you need in `Cargo.toml`:

```toml
[dependencies]
web-sys = { version = "0.3", features = [
    "console",
    "Document",
    "Element",
    "HtmlElement",
    "Node",
    "Window",
    "HtmlInputElement",
    "Event",
] }
```

## Error Handling

DOM operations can fail, so use `Result<(), JsValue>`:

```rust
#[wasm_bindgen]
pub fn safe_operation() -> Result<(), JsValue> {
    let element = document()
        .get_element_by_id("myId")
        .ok_or_else(|| JsValue::from_str("Element not found"))?;

    // Do something with element
    Ok(())
}
```

## Best Practices

1. **Minimize DOM Access**: Cache element references when possible
2. **Use web-sys Features**: Only enable the browser APIs you need
3. **Error Handling**: Always handle potential DOM operation failures
4. **Type Safety**: Use specific types like `HtmlInputElement` instead of `Element`
5. **Performance**: Batch DOM updates to avoid layout thrashing

## References

- [web-sys Documentation](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html)
- [DOM API Reference](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model)
