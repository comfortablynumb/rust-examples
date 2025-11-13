# wasm-pack: Building and Publishing WASM Packages

Demonstrates how to build production-ready WebAssembly packages using wasm-pack, including complex types, testing, and publishing.

## Concepts Covered

- Using wasm-pack for building WASM packages
- Working with complex data structures
- Serialization with serde
- Property getters and setters
- WASM testing with wasm-bindgen-test
- Package metadata and optimization
- Publishing to npm

## Code Examples

### Complex Types

```rust
#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct User {
    pub id: u32,
    name: String,
    email: String,
}

#[wasm_bindgen]
impl User {
    #[wasm_bindgen(constructor)]
    pub fn new(id: u32, name: String, email: String) -> User {
        User { id, name, email }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
```

### Stateful Structs

```rust
#[wasm_bindgen]
pub struct Statistics {
    data: Vec<f64>,
}

#[wasm_bindgen]
impl Statistics {
    pub fn add(&mut self, value: f64) {
        self.data.push(value);
    }

    pub fn mean(&self) -> Option<f64> {
        // Calculate mean
    }
}
```

## Building with wasm-pack

### Build Targets

```bash
# For web (ES modules)
wasm-pack build --target web

# For Node.js
wasm-pack build --target nodejs

# For bundlers (webpack, rollup, etc.)
wasm-pack build --target bundler

# With release optimizations
wasm-pack build --release --target web
```

### Build Output

After running `wasm-pack build`, you'll get a `pkg/` directory with:

```
pkg/
├── package.json           # npm package metadata
├── README.md             # Auto-generated from lib.rs docs
├── wasm_pack_example.js  # JavaScript bindings
├── wasm_pack_example.d.ts # TypeScript definitions
└── wasm_pack_example_bg.wasm # The WASM binary
```

## Testing

### Unit Tests

```bash
# Regular Rust tests
cargo test

# WASM-specific tests in browser
wasm-pack test --headless --firefox
wasm-pack test --headless --chrome

# Interactive browser test
wasm-pack test --firefox
```

### Writing WASM Tests

```rust
#[cfg(test)]
mod wasm_tests {
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_in_browser() {
        assert_eq!(1 + 1, 2);
    }
}
```

## Using in JavaScript/TypeScript

### JavaScript

```javascript
import init, { User, Statistics } from './pkg/wasm_pack_example.js';

async function main() {
    await init();

    // Create user
    const user = new User(1, "Alice", "alice@example.com");
    console.log(user.name);  // "Alice"
    console.log(user.greet());

    // Statistics
    const stats = new Statistics();
    stats.add_many([1, 2, 3, 4, 5]);
    console.log(stats.mean());  // 3.0
    console.log(stats.std_dev());
}

main();
```

### TypeScript

wasm-pack automatically generates TypeScript definitions:

```typescript
import init, { User, Statistics } from './pkg/wasm_pack_example';

async function main() {
    await init();

    const user: User = new User(1, "Bob", "bob@test.com");
    const stats: Statistics = new Statistics();
    stats.add_many([10, 20, 30]);

    const mean: number | undefined = stats.mean();
}

main();
```

## Publishing to npm

### 1. Configure package.json

The generated `pkg/package.json` can be customized in `Cargo.toml`:

```toml
[package]
name = "wasm-pack-example"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A sample WebAssembly package"
license = "MIT"
repository = "https://github.com/yourusername/wasm-pack-example"
```

### 2. Build for release

```bash
wasm-pack build --release --target bundler
```

### 3. Publish

```bash
cd pkg
npm publish
```

### 4. Install and use

```bash
npm install wasm-pack-example
```

```javascript
import init, { User } from 'wasm-pack-example';

init().then(() => {
    const user = new User(1, "Alice", "alice@example.com");
    console.log(user.greet());
});
```

## Optimization

### Cargo.toml Settings

```toml
[profile.release]
opt-level = "s"        # Optimize for size
lto = true             # Link-time optimization

[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-Oz", "--enable-mutable-globals"]
```

### Size Optimization Tips

1. **Use `opt-level = "s"` or `"z"`** for smaller binaries
2. **Enable LTO** (Link Time Optimization)
3. **Use `wasm-opt`** for additional optimization
4. **Minimize dependencies**
5. **Enable only needed web-sys features**

## Best Practices

1. **Documentation**: Use `///` doc comments - they appear in generated README
2. **Versioning**: Follow semver for package versions
3. **Testing**: Write both unit tests and WASM-specific tests
4. **TypeScript**: Leverage auto-generated type definitions
5. **Error Handling**: Return `Result<T, JsValue>` for fallible operations
6. **Memory**: Be mindful of memory passing between Rust and JS

## Common Patterns

### Getter/Setter Pattern

```rust
#[wasm_bindgen]
impl MyStruct {
    #[wasm_bindgen(getter)]
    pub fn field(&self) -> String {
        self.field.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_field(&mut self, value: String) {
        self.field = value;
    }
}
```

### Working with Arrays

```rust
#[wasm_bindgen]
pub fn process_array(data: Vec<f64>) -> Vec<f64> {
    data.iter().map(|x| x * 2.0).collect()
}
```

## References

- [wasm-pack Documentation](https://rustwasm.github.io/docs/wasm-pack/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Publishing to npm](https://docs.npmjs.com/cli/v8/commands/npm-publish)
