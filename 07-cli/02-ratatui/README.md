# Ratatui Terminal UI Dashboard

A comprehensive example of building interactive terminal user interfaces with ratatui, featuring a multi-tab dashboard with real-time data updates and keyboard navigation.

## Features Demonstrated

### 1. **Multiple Widget Types**
- **List Widget** - Server status list with color-coded states
- **Table Widget** - User management table with styled cells
- **Gauge Widget** - CPU, Memory, and Disk usage indicators
- **Chart Widget** - Real-time line chart with rolling data
- **BarChart Widget** - Service load visualization
- **Tabs Widget** - Multi-tab navigation interface
- **Paragraph Widget** - Formatted text display

### 2. **Interactive Features**
- Tab-based navigation (Tab/Shift+Tab)
- Up/Down arrow key navigation within lists and tables
- Real-time data updates (250ms tick rate)
- Color-coded status indicators
- Keyboard shortcuts

### 3. **UI Layout**
- Responsive layout using constraint-based sizing
- Header, content area, and footer sections
- Nested layouts for complex widget arrangements
- Percentage and fixed-size constraints

### 4. **State Management**
- Centralized application state
- Tick-based state updates
- Selection tracking for interactive widgets
- Frame counting and performance metrics

### 5. **Terminal Management**
- Raw mode for capturing key events
- Alternate screen buffer (preserves terminal history)
- Mouse capture support
- Proper cleanup on exit

## Usage

```bash
# Run the dashboard
cargo run

# The dashboard will fill your terminal window
```

### Keyboard Controls

- **Tab** - Navigate to next tab
- **Shift+Tab** - Navigate to previous tab
- **↑** (Up Arrow) - Move selection up (in Servers and Users tabs)
- **↓** (Down Arrow) - Move selection down (in Servers and Users tabs)
- **Q** or **Esc** - Quit the application

## Dashboard Tabs

### 1. Servers Tab
- Displays a list of servers with status
- Color-coded status indicators:
  - Green: Running
  - Yellow: Warning
  - Red: Error
  - Gray: Stopped
- Navigate with arrow keys
- Selected server highlighted

### 2. Users Tab
- Table view of user accounts
- Columns: Email, Name, Role, Status
- Sortable and navigable
- Active/Inactive status coloring
- Arrow key navigation

### 3. Metrics Tab
- Three system gauges:
  - CPU Usage (with dynamic color based on load)
  - Memory Usage
  - Disk Usage
- Real-time line chart
- Rolling 50-point data window
- Updates every 250ms

### 4. Performance Tab
- Bar chart showing service load
- Real-time statistics:
  - Frames rendered
  - Update rate
  - Performance status
- Dynamic data updates

## Architecture

### Application State

```rust
struct App {
    current_tab: usize,           // Active tab index
    list_items: Vec<String>,      // Server list data
    list_selected: usize,         // Selected server
    table_data: Vec<Vec<String>>, // User table data
    table_selected: usize,        // Selected user
    chart_data: Vec<(f64, f64)>,  // Chart coordinates
    cpu_usage: u16,               // Gauge values
    memory_usage: u16,
    disk_usage: u16,
    bar_data: Vec<(&'static str, u64)>, // Bar chart data
    frame_count: u64,             // Performance metrics
    should_quit: bool,            // Exit flag
}
```

### Event Loop

1. **Draw** - Render UI to terminal
2. **Poll** - Check for keyboard events (with timeout)
3. **Handle Input** - Process key presses
4. **Tick** - Update state at regular intervals
5. **Repeat** until quit signal

### Rendering Pipeline

```
ui()
├── render_header()      - Tab navigation
├── match current_tab
│   ├── render_servers_tab()    - List widget
│   ├── render_users_tab()      - Table widget
│   ├── render_metrics_tab()
│   │   ├── render_gauges()     - Three gauge widgets
│   │   └── render_chart()      - Line chart
│   └── render_performance_tab() - Bar chart + stats
└── render_footer()      - Help text
```

## Key Concepts

### Layout Constraints

Ratatui uses constraints to create responsive layouts:

```rust
Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),    // Fixed height
        Constraint::Min(0),       // Fill remaining space
        Constraint::Percentage(30), // 30% of space
    ])
    .split(area)
```

### Widget Styling

Rich styling options for visual feedback:

```rust
Style::default()
    .fg(Color::Yellow)
    .bg(Color::DarkGray)
    .add_modifier(Modifier::BOLD)
```

### Real-time Updates

The tick system enables smooth animations:

```rust
fn on_tick(&mut self) {
    // Update gauge values
    self.cpu_usage = update_value(self.cpu_usage);

    // Add new chart point
    self.chart_data.push((x, y));

    // Remove old data (rolling window)
    if self.chart_data.len() > 50 {
        self.chart_data.remove(0);
    }
}
```

### Terminal Lifecycle

Proper terminal management ensures clean operation:

```rust
// Setup
enable_raw_mode()?;
execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

// ... application runs ...

// Cleanup (even on error)
disable_raw_mode()?;
execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
```

## Customization Ideas

1. **Add More Tabs**
   - Logs viewer
   - Configuration panel
   - Help screen

2. **Enhanced Interactions**
   - Mouse click support
   - Text input fields
   - Popup dialogs

3. **Data Sources**
   - Connect to actual system metrics
   - Live log tailing
   - Database queries

4. **Visualization**
   - Sparklines
   - Histograms
   - Custom widgets

## Performance Considerations

- **Tick Rate**: 250ms provides smooth updates without excessive CPU usage
- **Data Limits**: Chart maintains rolling window of 50 points
- **Efficient Rendering**: Only redraws on events or ticks
- **Memory Management**: Bounded data structures prevent growth

## Dependencies

- **ratatui** - TUI framework
- **crossterm** - Terminal manipulation
- **rand** - Random data generation for demo

## Common Patterns

### Creating Interactive Lists

```rust
let items: Vec<ListItem> = data.iter()
    .map(|item| ListItem::new(item))
    .collect();

let list = List::new(items)
    .highlight_style(Style::default().bg(Color::DarkGray));
```

### Building Tables

```rust
let rows = data.iter().map(|row| {
    let cells = row.iter().map(|c| Cell::from(c));
    Row::new(cells)
});

let table = Table::new(rows, widths)
    .header(header_row);
```

### Dynamic Color Coding

```rust
fn status_color(value: u16) -> Color {
    match value {
        0..=50 => Color::Green,
        51..=80 => Color::Yellow,
        _ => Color::Red,
    }
}
```

## Troubleshooting

### Terminal Not Restored
If the application crashes and your terminal looks broken:
```bash
reset
```

### Colors Not Showing
Ensure your terminal supports 256 colors:
```bash
echo $TERM  # should be xterm-256color or similar
```

### Flickering
Reduce tick rate or optimize rendering logic.

## Learning Resources

- [Ratatui Documentation](https://docs.rs/ratatui/)
- [Ratatui Examples](https://github.com/ratatui-org/ratatui/tree/main/examples)
- [Crossterm Documentation](https://docs.rs/crossterm/)
- [TUI Design Patterns](https://ratatui.rs/concepts/)

## Production Considerations

For production TUI applications:

1. **Error Handling** - Always restore terminal state on panic
2. **Signal Handling** - Handle SIGTERM, SIGINT gracefully
3. **Logging** - Use file-based logging (not stdout)
4. **Testing** - Mock terminal for unit tests
5. **Accessibility** - Consider screen readers and alternative input methods
