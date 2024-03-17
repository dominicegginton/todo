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
    let list_widget_string = match app.local_list {
        true => "L",
        false => "G",
    };
    let mode_widget_string = match app.mode {
        Mode::Normal => "NORMAL",
        _ => "",
    };
    Paragraph::new(match app.mode {
        Mode::Insert => {
            let cursor = match app.selected_item {
                0 => "+",
                _ => "-",
            };
            format!(" {} {}", cursor, app.input.as_str())
        }
        _ => format!(" {} | {}", list_widget_string, mode_widget_string),
    })
    .set_style(match app.mode {
        Mode::Normal => Style::default().bg(Color::Blue).fg(Color::Black),
        Mode::Confirmation => Style::default().bg(Color::Red).fg(Color::White),
        Mode::Insert => Style::default().bg(Color::White).fg(Color::Black),
    })
}

fn list(app: &App) -> List {
    let mut items = Vec::new();
    for item in &app.items {
        let list_item = match item.complete {
            true => ListItem::new(format!("[x] {}", item.content))
                .set_style(Style::default().fg(Color::Blue).set_style(Modifier::BOLD)),
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
        Mode::Insert => {
            #[allow(clippy::cast_possible_truncation)]
            frame.set_cursor(
                status_area.x + app.cursor_position as u16 + 3,
                status_area.y,
            );
        }
    }
}
