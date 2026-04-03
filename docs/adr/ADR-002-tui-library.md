# ADR-004: TUI Library Selection

**Date**: 2026-04-02  
**Status**: Accepted  
**Deciders**: Agent  

## Context

heliosCLI requires a Terminal User Interface (TUI) for interactive operations like application monitoring, log streaming, and deployment visualization. The TUI library choice impacts performance, widget availability, and developer experience.

## Decision Drivers

- **Widget ecosystem**: Rich set of built-in widgets
- **Performance**: Render speed, memory usage
- **Async support**: Integration with tokio
- **Backend support**: Cross-platform terminal handling
- **Documentation**: Examples and tutorials
- **Maintenance**: Active development

## Options Considered

### Option A: ratatui

**GitHub**: [ratatui-org/ratatui](https://github.com/ratatui-org/ratatui)  
**Stars**: 11k+

**Key Facts**:
- Official successor to tui-rs (deprecated)
- Immediate mode rendering
- 20+ built-in widgets
- Async support via tokio integration

**Pros**:
- Active development (successor to tui-rs)
- Rich widget set (Block, List, Table, Chart, Canvas, etc.)
- Good performance (~1-5ms render)
- Strong community and documentation
- Works with crossterm (cross-platform)

**Cons**:
- Manual state management
- Learning curve for complex layouts
- No built-in event loop (must implement)

### Option B: cursive

**GitHub**: [gyscos/cursive](https://github.com/gyscos/cursive)
- View-based architecture (like Android/iOS)
- Higher-level abstractions

**Pros**:
- Higher-level API
- Built-in event handling
- Stack-based navigation

**Cons**:
- Less flexible than ratatui
- No async support
- Smaller ecosystem

**Verdict**: Less suitable for heliosCLI's needs

### Option C: tui-rs (Deprecated)

- Original TUI library
- No longer maintained
- Community migrated to ratatui

**Verdict**: Not an option (deprecated)

## Decision

**Adopt ratatui with crossterm backend**.

### Implementation Architecture

```rust
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        block::{Position, Title},
        canvas::{Canvas, Context, Label, Line as CanvasLine, Points, Rectangle},
        Axis, Bar, BarChart, BarGroup, Block, Borders, Cell, Chart, Clear, Dataset,
        Gauge, HighlightSpacing, LineGauge, List, ListItem, ListState, Padding,
        Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Sparkline,
        Table, Tabs, Wrap,
    },
    Frame, Terminal, TerminalOptions,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::{self, stdout, Stdout},
    panic,
    time::{Duration, Instant},
};

/// Main application state
pub struct App {
    /// Current view/page
    pub current_page: Page,
    /// List of applications
    pub apps: Vec<Application>,
    /// Selected application index
    pub selected_index: usize,
    /// Should the app quit
    pub should_quit: bool,
    /// Last key pressed
    pub last_key: Option<KeyCode>,
    /// Refresh tick
    pub tick_count: u64,
}

pub enum Page {
    Dashboard,
    AppList,
    AppDetail(String), // app name
    Logs(String),      // app name
    Settings,
}

pub struct Application {
    pub name: String,
    pub status: AppStatus,
    pub backend: String,
    pub image: String,
    pub ports: Vec<(u16, u16)>,
    pub uptime: Duration,
    pub cpu_percent: f64,
    pub memory_mb: u64,
}

pub enum AppStatus {
    Running,
    Stopped,
    Error(String),
    Deploying,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_page: Page::Dashboard,
            apps: Vec::new(),
            selected_index: 0,
            should_quit: false,
            last_key: None,
            tick_count: 0,
        }
    }
    
    pub fn next(&mut self) {
        if !self.apps.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.apps.len();
        }
    }
    
    pub fn previous(&mut self) {
        if !self.apps.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.apps.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }
}

/// Main UI loop
pub async fn run_tui() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Setup panic hook to restore terminal
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        original_hook(info);
    }));
    
    // Create app state
    let mut app = App::new();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);
    
    // Main loop
    loop {
        // Draw UI
        terminal.draw(|f| draw_ui(f, &mut app))?;
        
        // Handle events with timeout for periodic refresh
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key(&mut app, key.code).await?;
                }
            }
        }
        
        // Periodic refresh
        if last_tick.elapsed() >= tick_rate {
            app.tick_count += 1;
            refresh_data(&mut app).await?;
            last_tick = Instant::now();
        }
        
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

/// Draw the UI based on current page
fn draw_ui(f: &mut Frame, app: &mut App) {
    match app.current_page {
        Page::Dashboard => draw_dashboard(f, app),
        Page::AppList => draw_app_list(f, app),
        Page::AppDetail(ref name) => draw_app_detail(f, app, name),
        Page::Logs(ref name) => draw_logs(f, app, name),
        Page::Settings => draw_settings(f, app),
    }
}

/// Dashboard view with overview metrics
fn draw_dashboard(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(10), // Stats
            Constraint::Min(0),     // App list
            Constraint::Length(3),  // Footer
        ])
        .split(f.size());
    
    // Header
    let header = Paragraph::new("Helios Dashboard")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title("heliosCLI"));
    f.render_widget(header, chunks[0]);
    
    // Stats
    let running = app.apps.iter().filter(|a| matches!(a.status, AppStatus::Running)).count();
    let stopped = app.apps.iter().filter(|a| matches!(a.status, AppStatus::Stopped)).count();
    let errors = app.apps.iter().filter(|a| matches!(a.status, AppStatus::Error(_))).count();
    
    let stats_text = format!(
        "Running: {} | Stopped: {} | Errors: {} | Total: {}",
        running, stopped, errors, app.apps.len()
    );
    let stats = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title("Stats"));
    f.render_widget(stats, chunks[1]);
    
    // App list table
    let header_cells = ["Name", "Status", "Backend", "Uptime", "CPU", "Memory"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells).height(1).bottom_margin(1);
    
    let rows = app.apps.iter().enumerate().map(|(i, app)| {
        let status_style = match app.status {
            AppStatus::Running => Style::default().fg(Color::Green),
            AppStatus::Stopped => Style::default().fg(Color::Gray),
            AppStatus::Error(_) => Style::default().fg(Color::Red),
            AppStatus::Deploying => Style::default().fg(Color::Yellow),
        };
        
        let status_text = match &app.status {
            AppStatus::Error(e) => format!("Error: {}", e),
            _ => format!("{:?}", app.status),
        };
        
        let cells = vec![
            Cell::from(app.name.clone()),
            Cell::from(status_text).style(status_style),
            Cell::from(app.backend.clone()),
            Cell::from(format!("{:?}", app.uptime)),
            Cell::from(format!("{:.1}%", app.cpu_percent)),
            Cell::from(format!("{}MB", app.memory_mb)),
        ];
        
        Row::new(cells).height(1).style(
            if i == app.selected_index {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else {
                Style::default()
            }
        )
    });
    
    let table = Table::new(rows, [
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(15),
        Constraint::Percentage(20),
        Constraint::Percentage(10),
        Constraint::Percentage(15),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Applications"));
    
    f.render_widget(table, chunks[2]);
    
    // Footer with key hints
    let footer = Paragraph::new("q: Quit | ↑/↓: Navigate | Enter: Details | l: Logs | d: Deploy")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[3]);
}
```

### Key Dependencies

```toml
[dependencies]
# TUI
ratatui = { version = "0.26", features = ["crossterm"] }
crossterm = { version = "0.27", features = ["event-stream"] }

# Async
tokio = { version = "1.36", features = ["full"] }
tokio-util = "0.7"

# Utilities
unicode-width = "0.1"
textwrap = "0.16"
```

## Consequences

### Positive
- **Modern TUI**: Successor to tui-rs, actively maintained
- **Rich widgets**: Tables, charts, lists all built-in
- **Async support**: Works with tokio for real-time updates
- **Performance**: ~1-5ms render times, 60fps capable
- **Cross-platform**: crossterm works on Windows, macOS, Linux

### Negative
- **State management**: Manual (no built-in state management)
- **Learning curve**: Complex layouts require practice
- **Event loop**: Must implement own event handling

### Neutral
- **API stability**: Still evolving (0.x version)

## Widget Usage Guide

| Widget | Use Case | Example |
|--------|----------|---------|
| **Block** | Borders, titles | Container for other widgets |
| **Table** | Data display | App list, metrics |
| **List** | Selectable items | Menu, file list |
| **Paragraph** | Text display | Logs, help text |
| **Chart** | Graphs | CPU/memory over time |
| **Gauge** | Progress | Deployment progress |
| **Tabs** | Navigation | Page switching |

## References

- ratatui: https://github.com/ratatui-org/ratatui
- ratatui book: https://ratatui.rs/
- crossterm: https://github.com/crossterm-rs/crossterm
- heliosCLI SOTA Research: `docs/research/CLI_FRAMEWORKS_TUI_SOTA.md`

---

*This ADR will be updated as implementation progresses*
