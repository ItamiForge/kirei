use crate::scanner::{ScanResult, Scanner};
use crate::cleaner::Cleaner;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;
use std::time::Duration;

mod ui;

enum AppState {
    Scanning,
    Dashboard,
    List,
}

struct App {
    state: AppState,
    scanner: Scanner,
    results: Vec<ScanResult>,
    selected_category: usize,
    selected_item: usize,
    message: String,
    total_reclaimable: u64,
}

impl App {
    async fn new() -> Self {
        Self {
            state: AppState::Scanning,
            scanner: Scanner::new().await,
            results: Vec::new(),
            selected_category: 0,
            selected_item: 0,
            message: "Scanning...".to_string(),
            total_reclaimable: 0,
        }
    }

    async fn scan(&mut self) -> Result<()> {
        self.message = "Scanning...".to_string();
        self.results = self.scanner.scan_all().await?;
        self.update_totals();
        self.state = AppState::Dashboard;
        self.message = "Scan Complete".to_string();
        Ok(())
    }

    fn update_totals(&mut self) {
        self.total_reclaimable = self
            .results
            .iter()
            .flat_map(|r| &r.items)
            .filter(|i| i.selected)
            .map(|i| i.size)
            .sum();
    }

    fn toggle_selection(&mut self) {
        if let AppState::List = self.state {
            if let Some(category) = self.results.get_mut(self.selected_category) {
                if let Some(item) = category.items.get_mut(self.selected_item) {
                    item.selected = !item.selected;
                    self.update_totals();
                }
            }
        }
    }

    fn clean_selected(&mut self) -> Result<()> {
        let mut paths_to_clean = Vec::new();
        for res in &self.results {
            for item in &res.items {
                if item.selected {
                    paths_to_clean.push(item.path.clone());
                }
            }
        }

        if paths_to_clean.is_empty() {
             self.message = "Nothing selected".to_string();
             return Ok(());
        }

        self.message = format!("Cleaning {} items...", paths_to_clean.len());
        
        // This blocks the UI thread, ideally should be async or in thread
        match Cleaner::move_to_trash(&paths_to_clean) {
            Ok(size) => {
                self.message = format!("Moved to Trash: {}", human_bytes::human_bytes(size as f64));
                // Rescan after cleaning
                // simpler to just mark cleaned items as 0 size or remove them?
                // For now, let's just trigger a re-scan next loop if we could
            }
            Err(e) => {
                self.message = format!("Error: {}", e);
            }
        }
        
        Ok(())
    }
}

pub async fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new().await;

    // Initial scan
    app.scan().await?;

    let res = run_loop(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

async fn run_loop<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('r') => {
                             app.state = AppState::Scanning;
                             terminal.draw(|f| ui::draw(f, app))?; // draw scanning state
                             app.scan().await?;
                        },
                        KeyCode::Char('c') => {
                            app.clean_selected()?;
                             // Re-scan after clean to update UI
                             app.scan().await?;
                        }
                        KeyCode::Char(' ') => app.toggle_selection(),
                        KeyCode::Tab => {
                             match app.state {
                                 AppState::Dashboard => app.state = AppState::List,
                                 AppState::List => app.state = AppState::Dashboard,
                                 _ => {}
                             }
                        }
                        
                        // Navigation
                        KeyCode::Down | KeyCode::Char('j') => {
                            match app.state {
                                AppState::List => {
                                     if let Some(cat) = app.results.get(app.selected_category) {
                                         if app.selected_item + 1 < cat.items.len() {
                                             app.selected_item += 1;
                                         } else if app.selected_category + 1 < app.results.len() {
                                             app.selected_category += 1;
                                             app.selected_item = 0;
                                         }
                                     }
                                }
                                _ => {}
                            }
                        }
                         KeyCode::Up | KeyCode::Char('k') => {
                            match app.state {
                                AppState::List => {
                                     if app.selected_item > 0 {
                                         app.selected_item -= 1;
                                     } else if app.selected_category > 0 {
                                         app.selected_category -= 1;
                                         if let Some(cat) = app.results.get(app.selected_category) {
                                             app.selected_item = cat.items.len().saturating_sub(1);
                                         }
                                     }
                                }
                                _ => {}
                            }
                        }

                        _ => {}
                    }
                }
            }
        }
    }
}
