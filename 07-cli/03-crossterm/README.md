# Crossterm Terminal Manipulation

A comprehensive demonstration of direct terminal manipulation using crossterm, featuring multiple interactive demos including a fully functional text editor.

## Features Demonstrated

### 1. **Colors and Styling**
- Basic ANSI colors (8 colors)
- Background and foreground colors
- 24-bit RGB true color support
- Text attributes:
  - Bold, Dim, Italic
  - Underlined, Reverse
  - CrossedOut (strikethrough)
- Combined styling with the styled API

### 2. **Cursor Movement**
- Absolute cursor positioning with `MoveTo`
- Relative cursor movement
- Cursor show/hide
- Animated cursor movement demo
- Drawing at specific coordinates

### 3. **Raw Mode Input**
- Character-by-character input
- Key event handling
- Modifier keys (Ctrl, Shift, Alt)
- Special keys (arrows, Enter, Esc)
- Event polling with timeout

### 4. **Screen Management**
- Clear entire screen
- Clear specific regions
- Alternate screen buffer
- Screen size detection
- Responsive layouts

### 5. **Interactive Applications**
- Menu navigation with arrow keys
- Text editor with full editing capabilities
- Animation and real-time updates
- Status bars and UI elements

## Usage

```bash
# Run the demo application
cargo run

# Select from the menu:
# 1. Colors and Styling Demo
# 2. Cursor Movement Demo
# 3. Interactive Menu
# 4. Text Editor (full app)
# 5. Animation Demo
# 6. Exit
```

## Demo Descriptions

### Demo 1: Colors and Styling

Showcases all coloring and styling capabilities:
- Displays all 8 basic colors
- Shows foreground and background colors
- Demonstrates 24-bit RGB color gradients
- Exhibits all text attributes
- Examples of combined styling

### Demo 2: Cursor Movement

Visual demonstration of cursor control:
- Draws a box using box-drawing characters
- Animates cursor movement around the perimeter
- Shows precise positioning
- Writes text character by character

### Demo 3: Interactive Menu

Functional menu system:
- Arrow key navigation (↑/↓)
- Visual selection highlighting
- Enter to select, Esc to cancel
- Color-coded selection state

### Demo 4: Text Editor

Full-featured text editor with:
- Text input and editing
- Multi-line support
- Cursor movement (arrows, Home, End)
- Line operations (Enter, Backspace, Delete)
- Scrolling support
- Status bar showing position
- Keyboard shortcuts:
  - `Ctrl+Q` - Quit
  - `Ctrl+S` - Save (simulated)
  - Arrow keys - Navigate
  - Home/End - Line start/end

### Demo 5: Animation

Bouncing ball animation:
- Real-time rendering at 50ms intervals
- Physics simulation (velocity, bouncing)
- RGB color based on position
- Motion trail effect
- Frame counter
- Press Esc/Q to exit

## Key Concepts

### Raw Mode

Raw mode disables line buffering and special character processing:

```rust
// Enable raw mode for character-by-character input
terminal::enable_raw_mode()?;

// ... interactive code ...

// Always restore normal mode
terminal::disable_raw_mode()?;
```

### Alternate Screen

Alternate screen preserves terminal history:

```rust
// Switch to alternate screen
execute!(stdout, EnterAlternateScreen)?;

// ... application runs ...

// Return to main screen
execute!(stdout, LeaveAlternateScreen)?;
```

### Execute vs Queue

- **execute!** - Immediately writes to stdout
- **queue!** - Buffers commands for batch execution

```rust
// Immediate execution
execute!(stdout, MoveTo(0, 0), Print("Hello"))?;

// Buffered (flush later)
queue!(stdout, MoveTo(0, 0), Print("Hello"))?;
stdout.flush()?;
```

### Event Polling

Non-blocking input checking:

```rust
// Poll with timeout
if event::poll(Duration::from_millis(100))? {
    if let Event::Key(key) = event::read()? {
        // Handle key event
    }
}
```

### Color Types

```rust
// Basic colors
Color::Red
Color::Blue

// RGB colors (24-bit)
Color::Rgb { r: 255, g: 100, b: 50 }

// ANSI 256 color palette
Color::AnsiValue(42)
```

### Styled API

Convenient fluent styling:

```rust
// Method chaining
"Bold Red Text"
    .with(Color::Red)
    .attribute(Attribute::Bold)

"Cyan on Black"
    .with(Color::Cyan)
    .on(Color::Black)
```

## Text Editor Implementation

The text editor demonstrates production-grade patterns:

### State Management

```rust
struct TextEditor {
    lines: Vec<String>,      // Content
    cursor_x: usize,         // Column position
    cursor_y: usize,         // Line position
    scroll_offset: usize,    // Viewport scrolling
}
```

### Event Loop Pattern

```rust
loop {
    self.draw()?;              // Render UI

    if let Event::Key(key) = event::read()? {
        if !self.handle_key(key)? {
            break;              // Exit on Ctrl+Q
        }
    }
}
```

### Responsive Layout

```rust
let (width, height) = terminal::size()?;
let editor_height = height - 2;  // Reserve for status bar

// Adjust visible area
for line in lines.skip(scroll_offset).take(editor_height) {
    // Render visible lines
}
```

## Common Patterns

### Drawing Boxes

```rust
// Top border
execute!(stdout, MoveTo(x, y))?;
print!("┌{}┐", "─".repeat(width - 2));

// Sides
for i in 1..height - 1 {
    execute!(stdout, MoveTo(x, y + i))?;
    print!("│{}│", " ".repeat(width - 2));
}

// Bottom border
execute!(stdout, MoveTo(x, y + height - 1))?;
print!("└{}┘", "─".repeat(width - 2));
```

### Status Bars

```rust
execute!(
    stdout,
    MoveTo(0, height - 1),
    SetBackgroundColor(Color::DarkGrey),
    SetForegroundColor(Color::White),
    Print(format!(" {} ", status_text)),
    ResetColor
)?;
```

### Centering Text

```rust
let text = "Centered Message";
let center_x = (width - text.len() as u16) / 2;
let center_y = height / 2;
execute!(stdout, MoveTo(center_x, center_y), Print(text))?;
```

## Best Practices

### 1. Always Restore Terminal State

```rust
fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    // Even if there's an error, restore terminal
    let result = run_app();

    terminal::disable_raw_mode()?;
    result
}
```

### 2. Use Alternate Screen for Full-Screen Apps

```rust
execute!(stdout, EnterAlternateScreen)?;
// ... app code ...
execute!(stdout, LeaveAlternateScreen)?;
```

### 3. Flush After Queued Operations

```rust
queue!(stdout, MoveTo(x, y))?;
queue!(stdout, Print("text"))?;
stdout.flush()?;  // Actually write to terminal
```

### 4. Handle Terminal Resize

```rust
loop {
    let (width, height) = terminal::size()?;
    // Redraw based on current size
}
```

### 5. Efficient Rendering

```rust
// Clear only what changed
execute!(stdout, MoveTo(x, y), Print("new text"))?;

// Avoid full screen clears when possible
```

## Performance Tips

1. **Batch Operations** - Use `queue!` for multiple commands
2. **Minimal Redraws** - Only update changed regions
3. **Buffered Output** - Write once per frame
4. **Efficient Clear** - Clear specific lines, not entire screen

## Terminal Compatibility

Crossterm works across platforms:
- **Linux/Unix** - Full support for all features
- **macOS** - Full support
- **Windows** - Full support (via Windows Console API)

Some features may have limited support on older terminals:
- 24-bit RGB colors (may fall back to 256-color palette)
- Some text attributes (italic, strikethrough)
- Box-drawing characters (may appear as ASCII)

## Dependencies

- **crossterm** - Cross-platform terminal manipulation

## Troubleshooting

### Terminal Doesn't Restore
If the app crashes and leaves your terminal in raw mode:
```bash
reset
```

### Colors Don't Work
Check your terminal's color support:
```bash
echo $TERM
# Should show xterm-256color or similar
```

### Box Characters Appear Wrong
Ensure your terminal uses UTF-8 encoding.

## Learning Resources

- [Crossterm Documentation](https://docs.rs/crossterm/)
- [Crossterm Examples](https://github.com/crossterm-rs/crossterm/tree/master/examples)
- [Terminal Codes Reference](https://en.wikipedia.org/wiki/ANSI_escape_code)

## Production Considerations

For production terminal applications:

1. **Error Handling** - Always restore terminal on panic
2. **Signal Handling** - Handle SIGINT, SIGTERM gracefully
3. **Panic Hook** - Custom panic handler to restore terminal
4. **Testing** - Mock terminal for unit tests
5. **Documentation** - Clear keyboard shortcuts
6. **Accessibility** - Provide non-visual feedback options

## Next Steps

Try extending the demos:
- Add syntax highlighting to the text editor
- Create a file browser
- Build a terminal-based game
- Implement a process monitor
- Design custom UI widgets
