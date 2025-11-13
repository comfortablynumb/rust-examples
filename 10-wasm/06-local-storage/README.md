# Local Storage with WebAssembly

Demonstrates using browser localStorage and sessionStorage from Rust/WASM for persistent data.

## Concepts Covered

- localStorage API access from WASM
- sessionStorage for temporary data
- Serialization/deserialization with serde
- Persistent user preferences
- Todo list management
- Storage quota and limits

## Code Examples

### Basic Storage Operations

```rust
#[wasm_bindgen]
pub fn set_item(key: &str, value: &str) -> Result<(), JsValue> {
    local_storage()?.set_item(key, value)?;
    Ok(())
}

#[wasm_bindgen]
pub fn get_item(key: &str) -> Result<Option<String>, JsValue> {
    Ok(local_storage()?.get_item(key)?)
}
```

### Storing Complex Data

```rust
#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct UserPreferences {
    theme: String,
    language: String,
    notifications: bool,
}

#[wasm_bindgen]
impl UserPreferences {
    pub fn save(&self) -> Result<(), JsValue> {
        let json = serde_json::to_string(self)?;
        set_item("user_preferences", &json)?;
        Ok(())
    }

    pub fn load() -> Result<UserPreferences, JsValue> {
        match get_item("user_preferences")? {
            Some(json) => Ok(serde_json::from_str(&json)?),
            None => Ok(UserPreferences::new()),
        }
    }
}
```

### Todo List Example

```rust
#[wasm_bindgen]
pub struct TodoList {
    items: Vec<TodoItem>,
}

#[wasm_bindgen]
impl TodoList {
    pub fn add(&mut self, text: String) {
        self.items.push(TodoItem { text, completed: false });
    }

    pub fn save(&self) -> Result<(), JsValue> {
        let json = serde_json::to_string(&self.items)?;
        set_item("todo_list", &json)?;
        Ok(())
    }
}
```

## Building

```bash
wasm-pack build --target web
```

## Usage in JavaScript

```html
<!DOCTYPE html>
<html>
<head>
    <title>Local Storage Demo</title>
</head>
<body>
    <h1>User Preferences</h1>

    <select id="theme">
        <option value="light">Light</option>
        <option value="dark">Dark</option>
    </select>

    <button id="save">Save</button>
    <button id="load">Load</button>

    <script type="module">
        import init, {
            UserPreferences,
            TodoList
        } from './pkg/local_storage.js';

        async function run() {
            await init();

            let prefs = UserPreferences.new();

            document.getElementById('save').addEventListener('click', () => {
                const theme = document.getElementById('theme').value;
                prefs.set_theme(theme);
                prefs.save();
                alert('Preferences saved!');
            });

            document.getElementById('load').addEventListener('click', () => {
                prefs = UserPreferences.load();
                document.getElementById('theme').value = prefs.theme;
                alert('Preferences loaded!');
            });
        }

        run();
    </script>
</body>
</html>
```

## localStorage vs sessionStorage

### localStorage
- Persists even after browser is closed
- Shared across all tabs/windows of the same origin
- Typical limit: 5-10 MB per origin

```rust
set_item("key", "value");  // Uses localStorage
```

### sessionStorage
- Cleared when tab/window is closed
- Separate for each tab/window
- Same storage limit as localStorage

```rust
SessionStore::set("key", "value");  // Uses sessionStorage
```

## Storage Limits

```rust
#[wasm_bindgen]
pub fn get_storage_size() -> Result<usize, JsValue> {
    let storage = local_storage()?;
    let len = storage.length()?;

    let mut total_size = 0;
    for i in 0..len {
        if let Some(key) = storage.key(i)? {
            if let Some(value) = storage.get_item(&key)? {
                total_size += key.len() + value.len();
            }
        }
    }

    Ok(total_size)
}
```

## Best Practices

1. **Always Handle Errors**: Storage can fail (quota exceeded, private mode)
2. **Validate Data**: Check loaded data for corruption
3. **Use JSON**: Serialize complex data structures
4. **Namespace Keys**: Use prefixes to avoid conflicts
5. **Compress Data**: For large datasets, consider compression
6. **Check Availability**: Not all browsers/modes support storage

## Error Handling

```rust
pub fn safe_save(data: &str) -> Result<(), JsValue> {
    match set_item("my_key", data) {
        Ok(_) => Ok(()),
        Err(e) => {
            console::log_1(&"Storage failed".into());
            Err(e)
        }
    }
}
```

## Security Considerations

1. **No Sensitive Data**: localStorage is not encrypted
2. **XSS Vulnerable**: JavaScript can access localStorage
3. **Same-Origin Policy**: Data is accessible to all scripts on the same origin
4. **No Expiration**: Data persists until explicitly deleted

## Common Patterns

### Versioned Storage

```rust
#[derive(Serialize, Deserialize)]
struct VersionedData {
    version: u32,
    data: MyData,
}

impl VersionedData {
    fn load() -> Result<MyData, JsValue> {
        match get_item("my_data")? {
            Some(json) => {
                let versioned: VersionedData = serde_json::from_str(&json)?;
                if versioned.version == CURRENT_VERSION {
                    Ok(versioned.data)
                } else {
                    // Migrate data
                    Ok(migrate(versioned))
                }
            }
            None => Ok(MyData::default()),
        }
    }
}
```

### Auto-Save

```javascript
let todoList = TodoList.load();

// Auto-save on changes
function addTodo(text) {
    todoList.add(text);
    todoList.save();
}

// Save before page unload
window.addEventListener('beforeunload', () => {
    todoList.save();
});
```

## Alternative Storage Options

- **IndexedDB**: For larger datasets (no 5MB limit)
- **Cache API**: For offline app data
- **WebSQL**: Deprecated, avoid
- **Cookies**: For small data that needs server access

## References

- [Web Storage API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API)
- [localStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
- [Storage Quotas](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API/Browser_storage_limits_and_eviction_criteria)
