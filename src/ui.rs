use crate::app::{App, Mode};
use once_cell::sync::Lazy;
use ratatui::{
    prelude::*,
    widgets::{List, ListDirection, ListItem, ListState, Paragraph},
};

static LIST_STATE: Lazy<ListState> = Lazy::new(|| ListState::default());

fn layout() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
}

fn status_line(app: &App) -> Paragraph {
    match app.mode {
        Mode::Normal => Paragraph::new("NORMAL").set_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        ),
        Mode::Confirmation => Paragraph::new("CONFIRMATION | Delete? (y/n)").set_style(
            Style::default()
                .bg(Color::Red)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Mode::Edit => Paragraph::new(format!("- {}", app.input.as_str()))
            .style(Style::default().fg(Color::Black).bg(Color::White)),
        Mode::Insert => Paragraph::new(format!("+ {}", app.input.as_str()))
            .style(Style::default().fg(Color::Black).bg(Color::White)),
    }
}

fn list(app: &App) -> List {
    let mut items = Vec::new();
    for item in &app.items {
        let list_item = match item.complete {
            true => ListItem::new(format!("[x] {}", item.content)),
            false => ListItem::new(format!("[ ] {}", item.content)),
        };
        items.push(list_item);
    }

    List::new(items)
        .style(Style::default().fg(Color::White))
        .highlight_style(match app.mode {
            Mode::Normal => Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .set_style(Modifier::BOLD),
            Mode::Confirmation => Style::default(),
            Mode::Edit => Style::default(),
            Mode::Insert => Style::default(),
        })
        .direction(ListDirection::BottomToTop)
}

fn list_state(app: &App) -> ListState {
    let mut state = LIST_STATE.clone();
    state.select(Some(app.selected_item));
    state
}

pub fn render(app: &mut App, frame: &mut Frame) {
    let [list_area, status_area] = layout().areas(frame.size());
    let mut list_state = list_state(&app);

    frame.render_widget(status_line(&app), status_area);
    frame.render_stateful_widget(list(&app), list_area, &mut list_state);

    match app.mode {
        Mode::Normal => {}
        Mode::Confirmation => {}
        Mode::Edit => {
            #[allow(clippy::cast_possible_truncation)]
            frame.set_cursor(
                status_area.x + app.cursor_position as u16 + 2,
                status_area.y,
            );
        }
        Mode::Insert => {
            #[allow(clippy::cast_possible_truncation)]
            frame.set_cursor(
                status_area.x + app.cursor_position as u16 + 2,
                status_area.y,
            );
        }
    }
}
