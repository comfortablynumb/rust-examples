# Tauri Application

Build desktop apps with web frontends using Rust backend and HTML/CSS/JS frontend.

## What is Tauri?

Tauri creates lightweight desktop applications using:
- **Backend**: Rust for system access and business logic
- **Frontend**: Any web framework (React, Vue, vanilla JS, etc.)
- **Webview**: Native OS webview (not Electron's Chromium)

## Benefits

- **Small bundle size**: ~3-15MB vs 50-200MB for Electron
- **Memory efficient**: Uses system webview
- **Rust backend**: Fast, safe, and concurrent
- **Security**: Sandboxed by default
- **Cross-platform**: Windows, macOS, Linux

## Project Structure

```
tauri-app/
├── src/
│   └── main.rs          # Rust backend
├── src-tauri/
│   ├── tauri.conf.json  # Tauri config
│   └── Cargo.toml       # Rust dependencies
└── index.html           # Frontend (would be in a real project)
```

## Tauri Commands

Commands are Rust functions callable from JavaScript:

```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// In main:
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(...)
```

## Frontend Integration

```javascript
// JavaScript
import { invoke } from '@tauri-apps/api/tauri';

async function greet() {
    const result = await invoke('greet', { name: 'World' });
    console.log(result); // "Hello, World!"
}
```

## State Management

```rust
#[derive(Default)]
struct AppState {
    tasks: Mutex<Vec<Task>>,
}

#[tauri::command]
fn add_task(state: State<AppState>, title: String) -> Result<(), String> {
    let mut tasks = state.tasks.lock().unwrap();
    tasks.push(Task { title });
    Ok(())
}

// In main:
tauri::Builder::default()
    .manage(AppState::default())
    .invoke_handler(...)
```

## Building

### Development

```bash
npm install
npm run tauri dev
```

### Production

```bash
npm run tauri build
```

This creates:
- **Windows**: `.exe` installer
- **macOS**: `.dmg` or `.app`
- **Linux**: `.deb`, `.appimage`

## Complete Example

### Backend (src/main.rs)

```rust
#[tauri::command]
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
```

### Frontend (index.html)

```html
<!DOCTYPE html>
<html>
<body>
    <button id="btn">Add 2 + 3</button>
    <div id="result"></div>

    <script type="module">
        import { invoke } from '@tauri-apps/api/tauri';

        document.getElementById('btn').addEventListener('click', async () => {
            const result = await invoke('add_numbers', { a: 2, b: 3 });
            document.getElementById('result').textContent = result;
        });
    </script>
</body>
</html>
```

## Security

Tauri uses a security-first approach:

1. **Capability system**: Explicit permissions required
2. **Command allowlist**: Only enabled commands are exposed
3. **CSP**: Content Security Policy enabled by default
4. **No remote code**: Local-only by default

## Use Cases

- Desktop tools and utilities
- System tray applications
- Database GUIs
- Code editors
- Media players
- System monitors

## Tauri vs Electron

| Feature | Tauri | Electron |
|---------|-------|----------|
| Bundle Size | 3-15 MB | 50-200 MB |
| Memory | Lower | Higher |
| Backend | Rust | Node.js |
| Webview | System | Chromium |
| Startup | Faster | Slower |

## References

- [Tauri Documentation](https://tauri.app/)
- [Tauri API](https://tauri.app/v1/api/js/)
- [Tauri Examples](https://github.com/tauri-apps/tauri/tree/dev/examples)

## Note

This example shows the Rust backend structure. A complete Tauri app would include:
- `tauri.conf.json` configuration
- Frontend HTML/JS/CSS
- Build scripts (`package.json`)
