use crate::{app::App, file::FileMode};
use crate::input::InputMode;
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
    let file_mode = match app.file.mode {
        FileMode::Local => "L",
        FileMode::Global => "G"
    };

    Paragraph::new(match app.input.mode {
        InputMode::Insert => {
            let cursor = match app.list.selected_item {
                0 => "+",
                _ => "-",
            };
            format!(" {} | {} {}", file_mode, cursor, app.input.input)
        }
        _ => format!(" {}", file_mode),
    })
    .set_style(match app.input.mode {
        InputMode::Normal => Style::default().bg(Color::Blue).fg(Color::Black),
        InputMode::Insert => Style::default().bg(Color::White).fg(Color::Black),
    })
}

fn list(app: &App) -> List {
    let mut items = Vec::new();
    for item in &app.list.items {
        let list_item = match item.complete {
            true => ListItem::new(format!("[x] {}", item.content))
                .set_style(Style::default().fg(Color::Blue).set_style(Modifier::BOLD)),
            false => ListItem::new(format!("[ ] {}", item.content)),
        };
        items.push(list_item);
    }

    List::new(items)
        .style(Style::default().fg(Color::White))
        .highlight_style(match app.input.mode {
            InputMode::Normal => Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .set_style(Modifier::BOLD),
            InputMode::Insert => Style::default(),
        })
        .direction(ListDirection::BottomToTop)
}

fn list_state(app: &App) -> ListState {
    let mut state = LIST_STATE.clone();
    state.select(Some(app.list.selected_item));
    state
}

pub fn render(app: &mut App, frame: &mut Frame) {
    let [list_area, status_area] = layout().areas(frame.size());
    let mut list_state = list_state(&app);

    frame.render_widget(status_line(&app), status_area);
    frame.render_stateful_widget(list(&app), list_area, &mut list_state);

    match app.input.mode {
        InputMode::Normal => {}
        InputMode::Insert => {
            #[allow(clippy::cast_possible_truncation)]
            frame.set_cursor(
                status_area.x + app.input.cursor_position as u16 + 7,
                status_area.y,
            );
        }
    }
}
