//! Crossterm Terminal Manipulation Example
//!
//! This example demonstrates direct terminal manipulation using crossterm:
//! - Cursor movement and positioning
//! - Colors and text styling
//! - Raw mode input handling
//! - Screen clearing and drawing
//! - Interactive text editor implementation

use crossterm::{
    cursor::{self, MoveTo, MoveToColumn, MoveToNextLine},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor,
        SetForegroundColor, Stylize,
    },
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, stdout, Write},
    time::Duration,
};

/// Main application demonstrating various crossterm features
fn main() -> io::Result<()> {
    // Show menu
    show_menu()?;

    Ok(())
}

/// Display menu and handle demo selection
fn show_menu() -> io::Result<()> {
    let mut stdout = stdout();

    loop {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        println!("╔═══════════════════════════════════════════════════════════╗");
        println!("║         Crossterm Terminal Manipulation Demo             ║");
        println!("╚═══════════════════════════════════════════════════════════╝");
        println!();
        println!("Choose a demo:");
        println!();
        println!("  1. Colors and Styling Demo");
        println!("  2. Cursor Movement Demo");
        println!("  3. Interactive Menu");
        println!("  4. Text Editor (full app)");
        println!("  5. Animation Demo");
        println!("  6. Exit");
        println!();
        print!("Enter your choice (1-6): ");
        stdout.flush()?;

        // Read choice
        terminal::enable_raw_mode()?;
        let choice = loop {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                    if let KeyCode::Char(c) = code {
                        if c >= '1' && c <= '6' {
                            break c;
                        }
                    }
                }
            }
        };
        terminal::disable_raw_mode()?;

        println!("{}", choice);
        println!();

        match choice {
            '1' => colors_and_styling_demo()?,
            '2' => cursor_movement_demo()?,
            '3' => interactive_menu_demo()?,
            '4' => text_editor()?,
            '5' => animation_demo()?,
            '6' => {
                println!("Goodbye!");
                break;
            }
            _ => unreachable!(),
        }

        println!();
        println!("Press any key to return to menu...");
        terminal::enable_raw_mode()?;
        event::read()?;
        terminal::disable_raw_mode()?;
    }

    Ok(())
}

/// Demo 1: Colors and Styling
fn colors_and_styling_demo() -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    println!("═══ Colors and Styling Demo ═══");
    println!();

    // Basic colors
    println!("Basic Foreground Colors:");
    for color in &[
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ] {
        execute!(
            stdout,
            SetForegroundColor(*color),
            Print(format!("  {:?} ", color)),
            ResetColor
        )?;
    }
    println!();
    println!();

    // Background colors
    println!("Background Colors:");
    for color in &[
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ] {
        execute!(
            stdout,
            SetBackgroundColor(*color),
            SetForegroundColor(Color::Black),
            Print("  Text  "),
            ResetColor,
            Print(" ")
        )?;
    }
    println!();
    println!();

    // RGB colors
    println!("RGB Colors (24-bit true color):");
    for i in 0..16 {
        let r = (i * 16) as u8;
        let g = ((15 - i) * 16) as u8;
        let b = 128;
        execute!(
            stdout,
            SetBackgroundColor(Color::Rgb { r, g, b }),
            Print("  "),
            ResetColor
        )?;
    }
    println!();
    println!();

    // Text attributes
    println!("Text Attributes:");
    execute!(
        stdout,
        SetAttribute(Attribute::Bold),
        Print("  Bold text"),
        ResetColor
    )?;
    println!();

    execute!(
        stdout,
        SetAttribute(Attribute::Dim),
        Print("  Dim text"),
        ResetColor
    )?;
    println!();

    execute!(
        stdout,
        SetAttribute(Attribute::Italic),
        Print("  Italic text"),
        ResetColor
    )?;
    println!();

    execute!(
        stdout,
        SetAttribute(Attribute::Underlined),
        Print("  Underlined text"),
        ResetColor
    )?;
    println!();

    execute!(
        stdout,
        SetAttribute(Attribute::Reverse),
        Print("  Reversed text"),
        ResetColor
    )?;
    println!();

    execute!(
        stdout,
        SetAttribute(Attribute::CrossedOut),
        Print("  Crossed out text"),
        ResetColor
    )?;
    println!();
    println!();

    // Combined styling using styled API
    println!("Combined Styling (using styled API):");
    println!(
        "  {}",
        "Bold Red".with(Color::Red).attribute(Attribute::Bold)
    );
    println!(
        "  {}",
        "Italic Cyan on Black"
            .with(Color::Cyan)
            .on(Color::Black)
            .attribute(Attribute::Italic)
    );
    println!(
        "  {}",
        "Underlined Green"
            .with(Color::Green)
            .attribute(Attribute::Underlined)
    );

    Ok(())
}

/// Demo 2: Cursor Movement
fn cursor_movement_demo() -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    println!("═══ Cursor Movement Demo ═══");
    println!();
    println!("Watch the cursor move around...");
    println!();

    terminal::enable_raw_mode()?;

    // Draw a box
    let box_x = 10;
    let box_y = 5;
    let box_width = 40;
    let box_height = 10;

    // Draw box border
    execute!(stdout, MoveTo(box_x, box_y))?;
    print!("┌{}┐", "─".repeat(box_width - 2));

    for i in 1..box_height - 1 {
        execute!(stdout, MoveTo(box_x, box_y + i))?;
        print!("│{}│", " ".repeat(box_width - 2));
    }

    execute!(stdout, MoveTo(box_x, box_y + box_height - 1))?;
    print!("└{}┘", "─".repeat(box_width - 2));
    stdout.flush()?;

    // Move cursor around the box perimeter
    let mut positions = Vec::new();

    // Top edge
    for x in box_x + 1..box_x + box_width - 1 {
        positions.push((x, box_y));
    }
    // Right edge
    for y in box_y + 1..box_y + box_height - 1 {
        positions.push((box_x + box_width - 1, y));
    }
    // Bottom edge
    for x in (box_x + 1..box_x + box_width - 1).rev() {
        positions.push((x, box_y + box_height - 1));
    }
    // Left edge
    for y in (box_y + 1..box_y + box_height - 1).rev() {
        positions.push((box_x, y));
    }

    // Animate cursor movement
    for _ in 0..2 {
        // Two loops
        for (x, y) in &positions {
            execute!(stdout, MoveTo(*x, *y), Print("*"))?;
            stdout.flush()?;
            std::thread::sleep(Duration::from_millis(20));

            // Clear previous position (except corners and edges)
            if *x > box_x && *x < box_x + box_width - 1 && *y > box_y && *y < box_y + box_height - 1
            {
                execute!(stdout, MoveTo(*x, *y), Print(" "))?;
            }
        }
    }

    // Write message in center
    let msg = "Cursor Control!";
    let msg_x = box_x + (box_width - msg.len() as u16) / 2;
    let msg_y = box_y + box_height / 2;

    for (i, ch) in msg.chars().enumerate() {
        execute!(
            stdout,
            MoveTo(msg_x + i as u16, msg_y),
            SetForegroundColor(Color::Yellow),
            Print(ch),
            ResetColor
        )?;
        stdout.flush()?;
        std::thread::sleep(Duration::from_millis(100));
    }

    std::thread::sleep(Duration::from_secs(2));
    terminal::disable_raw_mode()?;

    Ok(())
}

/// Demo 3: Interactive Menu
fn interactive_menu_demo() -> io::Result<()> {
    let mut stdout = stdout();

    let options = vec![
        "New File",
        "Open File",
        "Save File",
        "Save As...",
        "Settings",
        "Exit",
    ];

    let mut selected = 0;

    terminal::enable_raw_mode()?;

    loop {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        println!("═══ Interactive Menu Demo ═══");
        println!();
        println!("Use ↑/↓ arrows to navigate, Enter to select, Esc to exit");
        println!();

        for (i, option) in options.iter().enumerate() {
            execute!(stdout, MoveToColumn(5))?;

            if i == selected {
                execute!(
                    stdout,
                    SetBackgroundColor(Color::Blue),
                    SetForegroundColor(Color::White),
                    SetAttribute(Attribute::Bold),
                    Print(format!(" ▶ {} ", option)),
                    ResetColor
                )?;
            } else {
                execute!(stdout, Print(format!("   {}  ", option)))?;
            }
            println!();
        }

        stdout.flush()?;

        // Handle input
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < options.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    terminal::disable_raw_mode()?;
                    println!();
                    println!("You selected: {}", options[selected]);
                    return Ok(());
                }
                KeyCode::Esc => {
                    terminal::disable_raw_mode()?;
                    println!();
                    println!("Menu cancelled");
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}

/// Demo 4: Simple Text Editor
fn text_editor() -> io::Result<()> {
    let mut editor = TextEditor::new();
    editor.run()?;
    Ok(())
}

/// Simple text editor implementation
struct TextEditor {
    lines: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
    scroll_offset: usize,
}

impl TextEditor {
    fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
            scroll_offset: 0,
        }
    }

    fn run(&mut self) -> io::Result<()> {
        let mut stdout = stdout();

        terminal::enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, cursor::Show)?;

        loop {
            self.draw(&mut stdout)?;

            if let Event::Key(key_event) = event::read()? {
                if !self.handle_key(key_event)? {
                    break;
                }
            }
        }

        execute!(stdout, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }

    fn draw(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        let (width, height) = terminal::size()?;
        let editor_height = height - 2; // Reserve space for status bar

        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        // Draw content
        for (i, line) in self
            .lines
            .iter()
            .skip(self.scroll_offset)
            .take(editor_height as usize)
            .enumerate()
        {
            execute!(stdout, MoveTo(0, i as u16))?;
            let display_line = if line.len() > width as usize {
                &line[..width as usize]
            } else {
                line
            };
            print!("{}", display_line);
        }

        // Draw status bar
        execute!(
            stdout,
            MoveTo(0, height - 2),
            SetBackgroundColor(Color::DarkGrey),
            SetForegroundColor(Color::White),
            Print("─".repeat(width as usize)),
            ResetColor
        )?;

        execute!(
            stdout,
            MoveTo(0, height - 1),
            SetBackgroundColor(Color::DarkGrey),
            SetForegroundColor(Color::White)
        )?;
        print!(
            " Line {}/{} | Col {} | Ctrl+Q: Quit | Ctrl+S: Save (simulated)",
            self.cursor_y + 1,
            self.lines.len(),
            self.cursor_x + 1
        );
        execute!(stdout, ResetColor)?;

        // Position cursor
        let screen_y = (self.cursor_y - self.scroll_offset) as u16;
        execute!(stdout, MoveTo(self.cursor_x as u16, screen_y))?;

        stdout.flush()?;
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> io::Result<bool> {
        match (key.code, key.modifiers) {
            // Quit
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => return Ok(false),

            // Save (simulated)
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                // In a real editor, save to file here
            }

            // Navigation
            (KeyCode::Left, _) => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.lines[self.cursor_y].len();
                }
            }
            (KeyCode::Right, _) => {
                let line_len = self.lines[self.cursor_y].len();
                if self.cursor_x < line_len {
                    self.cursor_x += 1;
                } else if self.cursor_y < self.lines.len() - 1 {
                    self.cursor_y += 1;
                    self.cursor_x = 0;
                }
            }
            (KeyCode::Up, _) => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    let line_len = self.lines[self.cursor_y].len();
                    if self.cursor_x > line_len {
                        self.cursor_x = line_len;
                    }
                }
            }
            (KeyCode::Down, _) => {
                if self.cursor_y < self.lines.len() - 1 {
                    self.cursor_y += 1;
                    let line_len = self.lines[self.cursor_y].len();
                    if self.cursor_x > line_len {
                        self.cursor_x = line_len;
                    }
                }
            }

            // Home/End
            (KeyCode::Home, _) => self.cursor_x = 0,
            (KeyCode::End, _) => self.cursor_x = self.lines[self.cursor_y].len(),

            // Enter - new line
            (KeyCode::Enter, _) => {
                let current_line = &self.lines[self.cursor_y];
                let new_line = current_line[self.cursor_x..].to_string();
                self.lines[self.cursor_y] = current_line[..self.cursor_x].to_string();
                self.cursor_y += 1;
                self.lines.insert(self.cursor_y, new_line);
                self.cursor_x = 0;
            }

            // Backspace
            (KeyCode::Backspace, _) => {
                if self.cursor_x > 0 {
                    self.lines[self.cursor_y].remove(self.cursor_x - 1);
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    let current_line = self.lines.remove(self.cursor_y);
                    self.cursor_y -= 1;
                    self.cursor_x = self.lines[self.cursor_y].len();
                    self.lines[self.cursor_y].push_str(&current_line);
                }
            }

            // Delete
            (KeyCode::Delete, _) => {
                let line_len = self.lines[self.cursor_y].len();
                if self.cursor_x < line_len {
                    self.lines[self.cursor_y].remove(self.cursor_x);
                } else if self.cursor_y < self.lines.len() - 1 {
                    let next_line = self.lines.remove(self.cursor_y + 1);
                    self.lines[self.cursor_y].push_str(&next_line);
                }
            }

            // Character input
            (KeyCode::Char(c), _) => {
                self.lines[self.cursor_y].insert(self.cursor_x, c);
                self.cursor_x += 1;
            }

            _ => {}
        }

        // Adjust scroll
        let (_, height) = terminal::size()?;
        let editor_height = height - 2;

        if self.cursor_y < self.scroll_offset {
            self.scroll_offset = self.cursor_y;
        } else if self.cursor_y >= self.scroll_offset + editor_height as usize {
            self.scroll_offset = self.cursor_y - editor_height as usize + 1;
        }

        Ok(true)
    }
}

/// Demo 5: Animation
fn animation_demo() -> io::Result<()> {
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All), cursor::Hide)?;

    let (width, height) = terminal::size()?;

    // Bouncing ball animation
    let mut ball_x = width / 2;
    let mut ball_y = height / 2;
    let mut vel_x: i16 = 1;
    let mut vel_y: i16 = 1;

    let mut frame = 0;

    loop {
        // Check for exit
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                if matches!(code, KeyCode::Esc | KeyCode::Char('q')) {
                    break;
                }
            }
        }

        // Clear previous frame
        execute!(stdout, Clear(ClearType::All))?;

        // Draw border
        execute!(stdout, MoveTo(0, 0))?;
        print!("╔{}╗", "═".repeat(width as usize - 2));
        for y in 1..height - 1 {
            execute!(stdout, MoveTo(0, y))?;
            print!("║");
            execute!(stdout, MoveTo(width - 1, y))?;
            print!("║");
        }
        execute!(stdout, MoveTo(0, height - 1))?;
        print!("╚{}╝", "═".repeat(width as usize - 2));

        // Update ball position
        ball_x = (ball_x as i16 + vel_x) as u16;
        ball_y = (ball_y as i16 + vel_y) as u16;

        // Bounce off walls
        if ball_x <= 1 || ball_x >= width - 2 {
            vel_x = -vel_x;
        }
        if ball_y <= 1 || ball_y >= height - 2 {
            vel_y = -vel_y;
        }

        // Draw ball with color based on position
        let color = Color::Rgb {
            r: ((ball_x as f32 / width as f32) * 255.0) as u8,
            g: ((ball_y as f32 / height as f32) * 255.0) as u8,
            b: 200,
        };

        execute!(
            stdout,
            MoveTo(ball_x, ball_y),
            SetForegroundColor(color),
            Print("●"),
            ResetColor
        )?;

        // Draw trail
        let trail_len = 5;
        for i in 1..=trail_len {
            let trail_x = (ball_x as i16 - vel_x * i) as u16;
            let trail_y = (ball_y as i16 - vel_y * i) as u16;

            if trail_x > 0 && trail_x < width - 1 && trail_y > 0 && trail_y < height - 1 {
                let alpha = 255 - (i * 50) as u8;
                execute!(
                    stdout,
                    MoveTo(trail_x, trail_y),
                    SetForegroundColor(Color::Rgb {
                        r: alpha,
                        g: alpha,
                        b: alpha
                    }),
                    Print("·"),
                    ResetColor
                )?;
            }
        }

        // Draw info
        execute!(
            stdout,
            MoveTo(2, height - 2),
            SetForegroundColor(Color::Yellow),
            Print(format!("Frame: {} | Press Esc/Q to exit", frame)),
            ResetColor
        )?;

        stdout.flush()?;
        frame += 1;

        std::thread::sleep(Duration::from_millis(50));
    }

    execute!(stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
