//! Ratatui Terminal UI Dashboard Example
//!
//! This example demonstrates building a feature-rich terminal UI application
//! using ratatui. It includes:
//! - Multiple widgets: lists, tables, charts, gauges, and tabs
//! - Keyboard navigation and input handling
//! - Real-time data updates
//! - Responsive layout design
//! - State management

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span, Text},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, List, ListItem,
        Paragraph, Row, Table, Tabs, Wrap,
    },
    Frame, Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

/// Application state and data
struct App {
    /// Current selected tab
    current_tab: usize,

    /// List widget state
    list_items: Vec<String>,
    list_selected: usize,

    /// Table widget data
    table_data: Vec<Vec<String>>,
    table_selected: usize,

    /// Chart data points
    chart_data: Vec<(f64, f64)>,

    /// Gauge values (0-100)
    cpu_usage: u16,
    memory_usage: u16,
    disk_usage: u16,

    /// Bar chart data
    bar_data: Vec<(&'static str, u64)>,

    /// Performance metrics
    frame_count: u64,
    last_tick: Instant,

    /// Exit flag
    should_quit: bool,
}

impl App {
    /// Create a new application instance with initial data
    fn new() -> App {
        let mut rng = rand::thread_rng();

        // Initialize list items
        let list_items = vec![
            "Server 1: Running".to_string(),
            "Server 2: Running".to_string(),
            "Server 3: Warning".to_string(),
            "Server 4: Running".to_string(),
            "Server 5: Error".to_string(),
            "Server 6: Running".to_string(),
            "Server 7: Stopped".to_string(),
            "Server 8: Running".to_string(),
        ];

        // Initialize table data
        let table_data = vec![
            vec!["alice@example.com".to_string(), "Alice".to_string(), "Admin".to_string(), "Active".to_string()],
            vec!["bob@example.com".to_string(), "Bob".to_string(), "User".to_string(), "Active".to_string()],
            vec!["carol@example.com".to_string(), "Carol".to_string(), "User".to_string(), "Inactive".to_string()],
            vec!["dave@example.com".to_string(), "Dave".to_string(), "Moderator".to_string(), "Active".to_string()],
            vec!["eve@example.com".to_string(), "Eve".to_string(), "User".to_string(), "Active".to_string()],
        ];

        // Initialize chart data
        let chart_data: Vec<(f64, f64)> = (0..50)
            .map(|i| {
                let x = i as f64;
                let y = (x * 0.1).sin() * 20.0 + 50.0 + rng.gen_range(-5.0..5.0);
                (x, y)
            })
            .collect();

        // Initialize bar chart data
        let bar_data = vec![
            ("API", 65),
            ("Web", 80),
            ("Database", 45),
            ("Cache", 90),
            ("Queue", 55),
        ];

        App {
            current_tab: 0,
            list_items,
            list_selected: 0,
            table_data,
            table_selected: 0,
            chart_data,
            cpu_usage: rng.gen_range(30..70),
            memory_usage: rng.gen_range(40..80),
            disk_usage: rng.gen_range(20..60),
            bar_data,
            frame_count: 0,
            last_tick: Instant::now(),
            should_quit: false,
        }
    }

    /// Update application state (simulating real-time data)
    fn on_tick(&mut self) {
        let mut rng = rand::thread_rng();

        // Update gauge values with slight random changes
        self.cpu_usage = (self.cpu_usage as i16 + rng.gen_range(-5..5))
            .max(0)
            .min(100) as u16;
        self.memory_usage = (self.memory_usage as i16 + rng.gen_range(-3..3))
            .max(0)
            .min(100) as u16;
        self.disk_usage = (self.disk_usage as i16 + rng.gen_range(-2..2))
            .max(0)
            .min(100) as u16;

        // Update chart data (rolling window)
        let last_x = self.chart_data.last().map(|(x, _)| *x).unwrap_or(0.0);
        let new_x = last_x + 1.0;
        let new_y = (new_x * 0.1).sin() * 20.0 + 50.0 + rng.gen_range(-5.0..5.0);

        self.chart_data.push((new_x, new_y));
        if self.chart_data.len() > 50 {
            self.chart_data.remove(0);
        }

        // Update bar chart data
        for (_, value) in &mut self.bar_data {
            *value = (*value as i16 + rng.gen_range(-5..5))
                .max(10)
                .min(100) as u64;
        }

        self.frame_count += 1;
    }

    /// Handle keyboard input
    fn handle_input(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Tab => {
                self.current_tab = (self.current_tab + 1) % 4;
            }
            KeyCode::BackTab => {
                self.current_tab = if self.current_tab == 0 { 3 } else { self.current_tab - 1 };
            }
            KeyCode::Up => match self.current_tab {
                0 => {
                    if self.list_selected > 0 {
                        self.list_selected -= 1;
                    }
                }
                1 => {
                    if self.table_selected > 0 {
                        self.table_selected -= 1;
                    }
                }
                _ => {}
            },
            KeyCode::Down => match self.current_tab {
                0 => {
                    if self.list_selected < self.list_items.len() - 1 {
                        self.list_selected += 1;
                    }
                }
                1 => {
                    if self.table_selected < self.table_data.len() - 1 {
                        self.table_selected += 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    // Main event loop
    loop {
        terminal.draw(|f| ui(f, &app))?;

        // Poll for events with timeout
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Only process KeyPress events to avoid double-triggering
                if key.kind == KeyEventKind::Press {
                    app.handle_input(key.code);
                }
            }
        }

        // Update state on tick
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        // Exit check
        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

/// Main UI rendering function
fn ui(f: &mut Frame, app: &App) {
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header with tabs
            Constraint::Min(0),     // Content area
            Constraint::Length(1),  // Footer
        ])
        .split(f.area());

    // Render header with tabs
    render_header(f, chunks[0], app);

    // Render content based on selected tab
    match app.current_tab {
        0 => render_servers_tab(f, chunks[1], app),
        1 => render_users_tab(f, chunks[1], app),
        2 => render_metrics_tab(f, chunks[1], app),
        3 => render_performance_tab(f, chunks[1], app),
        _ => {}
    }

    // Render footer
    render_footer(f, chunks[2]);
}

/// Render the header with tab navigation
fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let titles = vec!["Servers", "Users", "Metrics", "Performance"];
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Dashboard"))
        .select(app.current_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        );
    f.render_widget(tabs, area);
}

/// Render the Servers tab with a list widget
fn render_servers_tab(f: &mut Frame, area: Rect, app: &App) {
    // Create list items with color coding based on status
    let items: Vec<ListItem> = app
        .list_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if item.contains("Error") {
                Style::default().fg(Color::Red)
            } else if item.contains("Warning") {
                Style::default().fg(Color::Yellow)
            } else if item.contains("Stopped") {
                Style::default().fg(Color::Gray)
            } else {
                Style::default().fg(Color::Green)
            };

            let content = if i == app.list_selected {
                format!("> {}", item)
            } else {
                format!("  {}", item)
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Server Status (↑/↓ to navigate)")
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray)
        );

    f.render_widget(list, area);
}

/// Render the Users tab with a table widget
fn render_users_tab(f: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["Email", "Name", "Role", "Status"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.table_data.iter().enumerate().map(|(i, row_data)| {
        let height = 1;
        let cells = row_data.iter().map(|c| {
            let style = if c == "Active" {
                Style::default().fg(Color::Green)
            } else if c == "Inactive" {
                Style::default().fg(Color::Gray)
            } else {
                Style::default()
            };
            Cell::from(c.as_str()).style(style)
        });

        let row = Row::new(cells).height(height);

        if i == app.table_selected {
            row.style(Style::default().bg(Color::DarkGray))
        } else {
            row
        }
    });

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("User Management (↑/↓ to navigate)")
    );

    f.render_widget(table, area);
}

/// Render the Metrics tab with gauges and charts
fn render_metrics_tab(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),  // Gauges
            Constraint::Min(0),      // Chart
        ])
        .split(area);

    // Render gauges
    render_gauges(f, chunks[0], app);

    // Render line chart
    render_chart(f, chunks[1], app);
}

/// Render system gauges (CPU, Memory, Disk)
fn render_gauges(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);

    // CPU gauge
    let cpu_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("CPU Usage"))
        .gauge_style(
            Style::default()
                .fg(gauge_color(app.cpu_usage))
                .bg(Color::Black)
        )
        .percent(app.cpu_usage)
        .label(format!("{}%", app.cpu_usage));
    f.render_widget(cpu_gauge, chunks[0]);

    // Memory gauge
    let memory_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Memory Usage"))
        .gauge_style(
            Style::default()
                .fg(gauge_color(app.memory_usage))
                .bg(Color::Black)
        )
        .percent(app.memory_usage)
        .label(format!("{}%", app.memory_usage));
    f.render_widget(memory_gauge, chunks[1]);

    // Disk gauge
    let disk_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Disk Usage"))
        .gauge_style(
            Style::default()
                .fg(gauge_color(app.disk_usage))
                .bg(Color::Black)
        )
        .percent(app.disk_usage)
        .label(format!("{}%", app.disk_usage));
    f.render_widget(disk_gauge, chunks[2]);
}

/// Helper function to determine gauge color based on value
fn gauge_color(value: u16) -> Color {
    if value >= 80 {
        Color::Red
    } else if value >= 60 {
        Color::Yellow
    } else {
        Color::Green
    }
}

/// Render line chart showing data over time
fn render_chart(f: &mut Frame, area: Rect, app: &App) {
    let datasets = vec![Dataset::default()
        .name("Response Time")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .data(&app.chart_data)];

    let x_bounds = app.chart_data.first().and_then(|(x, _)| Some(*x)).unwrap_or(0.0);
    let x_max = app.chart_data.last().and_then(|(x, _)| Some(*x)).unwrap_or(50.0);

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Real-time Metrics")
        )
        .x_axis(
            Axis::default()
                .title("Time")
                .style(Style::default().fg(Color::Gray))
                .bounds([x_bounds, x_max])
        )
        .y_axis(
            Axis::default()
                .title("Value")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 100.0])
                .labels(vec!["0".into(), "50".into(), "100".into()])
        );

    f.render_widget(chart, area);
}

/// Render the Performance tab with bar chart
fn render_performance_tab(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(70),  // Bar chart
            Constraint::Percentage(30),  // Stats
        ])
        .split(area);

    // Render bar chart
    let bar_chart = BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Service Load"))
        .data(&app.bar_data)
        .bar_width(9)
        .bar_gap(2)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));

    f.render_widget(bar_chart, chunks[0]);

    // Render statistics
    let stats_text = vec![
        Line::from(vec![
            Span::styled("Frames Rendered: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", app.frame_count),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            ),
        ]),
        Line::from(vec![
            Span::styled("Update Rate: ", Style::default().fg(Color::Gray)),
            Span::styled(
                "250ms",
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Performance: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(
                "Excellent",
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            ),
        ]),
    ];

    let stats_paragraph = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title("Statistics"))
        .wrap(Wrap { trim: true });

    f.render_widget(stats_paragraph, chunks[1]);
}

/// Render the footer with help text
fn render_footer(f: &mut Frame, area: Rect) {
    let text = vec![Line::from(vec![
        Span::styled(" Tab", Style::default().fg(Color::Yellow)),
        Span::raw(": Next | "),
        Span::styled("Shift+Tab", Style::default().fg(Color::Yellow)),
        Span::raw(": Previous | "),
        Span::styled("↑/↓", Style::default().fg(Color::Yellow)),
        Span::raw(": Navigate | "),
        Span::styled("Q/Esc", Style::default().fg(Color::Red)),
        Span::raw(": Quit "),
    ])];

    let paragraph = Paragraph::new(text)
        .style(Style::default().bg(Color::DarkGray))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}
