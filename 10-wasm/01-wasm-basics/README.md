# WebAssembly Basics

A simple introduction to compiling Rust to WebAssembly (WASM) and exposing functions to JavaScript.

## Concepts Covered

- Compiling Rust to WebAssembly
- Using `wasm-bindgen` for JS interop
- Exporting functions with `#[wasm_bindgen]`
- Handling different data types (integers, strings, booleans)
- Building optimized WASM binaries

## Code Examples

### Exporting Simple Functions

```rust
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

The `#[wasm_bindgen]` attribute macro marks functions for export to JavaScript.

### String Handling

```rust
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

`wasm-bindgen` handles string conversion between Rust and JavaScript automatically.

### Complex Computations

```rust
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

WASM is excellent for computationally intensive tasks that benefit from Rust's performance.

## Building

### Prerequisites

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Or with cargo
cargo install wasm-pack
```

### Build Commands

```bash
# Build for web
wasm-pack build --target web

# Build for Node.js
wasm-pack build --target nodejs

# Build for bundlers (webpack, etc.)
wasm-pack build --target bundler
```

### Running Tests

```bash
cargo test
wasm-pack test --headless --firefox
```

## Using in JavaScript

### Web (ES Modules)

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>WASM Basics</title>
</head>
<body>
    <script type="module">
        import init, { add, greet, fibonacci } from './pkg/wasm_basics.js';

        async function run() {
            await init();

            console.log(add(5, 3));  // 8
            console.log(greet("World"));  // Hello, World! Welcome to WebAssembly with Rust.
            console.log(fibonacci(10));  // 55
        }

        run();
    </script>
</body>
</html>
```

### Node.js

```javascript
const { add, greet, fibonacci } = require('./pkg/wasm_basics.js');

console.log(add(5, 3));
console.log(greet("World"));
console.log(fibonacci(10));
```

## Performance Considerations

- WASM has near-native performance
- Small binary sizes with optimization
- No garbage collection pauses
- Ideal for CPU-intensive tasks like:
  - Mathematical computations
  - Image/video processing
  - Cryptography
  - Game engines

## Key Points

1. **crate-type**: Must include `"cdylib"` for WASM compilation
2. **Optimization**: Use `opt-level = "s"` and `lto = true` for smaller binaries
3. **wasm-bindgen**: Handles the glue code between Rust and JavaScript
4. **Memory Safety**: Rust's guarantees extend to WASM, preventing common web vulnerabilities

## References

- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [MDN WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly)
