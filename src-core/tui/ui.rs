use super::{App, AppState};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
};
use human_bytes::human_bytes;

pub fn draw(frame: &mut Frame, app: &App) {
    if let AppState::Scanning = app.state {
        let block = Block::default().borders(Borders::ALL).title("Kirei");
        let gauge = Gauge::default()
            .block(block)
            .gauge_style(Style::default().fg(Color::Cyan))
            .percent(50) // Fake 50% for indeterminate
            .label("Scanning system...");
        
        let area = centered_rect(60, 20, frame.area());
        frame.render_widget(gauge, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header/Stats
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Status Bar
        ])
        .split(frame.area());

    // Header
    let total_str = human_bytes(app.total_reclaimable as f64);
    let title = Paragraph::new(format!(" KIREI - System Cleaner | Reclaimable: {} ", total_str))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Main Content
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // List
            Constraint::Percentage(50), // Details
        ])
        .split(chunks[1]);

    // List View
    let mut list_items = Vec::new();
    
    for (cat_idx, category) in app.results.iter().enumerate() {
        // Category Header
        list_items.push(ListItem::new(Line::from(vec![
            Span::styled(format!("{:?}", category.category), Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
            Span::raw(format!(" ({})", human_bytes(category.total_size as f64))),
        ])));

        for (item_idx, item) in category.items.iter().enumerate() {
            let is_selected_cursor = app.selected_category == cat_idx && app.selected_item == item_idx;
            let checkmark = if item.selected { "[x]" } else { "[ ]" };
            
            let style = if is_selected_cursor {
                Style::default().fg(Color::Black).bg(Color::Cyan)
            } else {
                match app.state {
                     AppState::List => Style::default(),
                     _ => Style::default().fg(Color::DarkGray),
                }
            };

            let name = if item.name.len() > 30 {
                format!("{}...", &item.name[..27])
            } else {
                item.name.clone()
            };

            list_items.push(ListItem::new(format!("  {} {:<30} {}", checkmark, name, human_bytes(item.size as f64))).style(style));
        }
    }

    let list_block = Block::default()
        .borders(Borders::ALL)
        .title(" Items ")
        .border_style(match app.state {
            AppState::List => Style::default().fg(Color::Cyan),
            _ => Style::default(),
        });

    let list = List::new(list_items)
        .block(list_block)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    
    frame.render_widget(list, main_chunks[0]);

    // Details View
    let details_block = Block::default()
        .borders(Borders::ALL)
        .title(" Details ");

    let details_content = if let Some(cat) = app.results.get(app.selected_category) {
        if let Some(item) = cat.items.get(app.selected_item) {
             vec![
                Line::from(vec![Span::styled("Path: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(item.path.to_string_lossy())]),
                Line::from(""),
                Line::from(vec![Span::styled("Size: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(human_bytes(item.size as f64))]),
                Line::from(""),
                Line::from(vec![Span::styled("Description: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(&item.description)]),
            ]
        } else {
            vec![Line::from("Select an item")]
        }
    } else {
         vec![Line::from("No items")]
    };

    let details = Paragraph::new(details_content)
        .block(details_block)
        .wrap(Wrap { trim: true });
    
    frame.render_widget(details, main_chunks[1]);


    // Status Bar
    let status_text = match app.state {
        AppState::Scanning => "Scanning...",
        _ => " [Space]Select [c]Clean [r]Rescan [Tab]View [q]Quit",
    };
    
    let status = Paragraph::new(format!("{} | {}", status_text, app.message))
        .style(Style::default().bg(Color::Blue).fg(Color::White));
    frame.render_widget(status, chunks[2]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
