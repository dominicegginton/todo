use crate::app::{App, Mode};
use once_cell::sync::Lazy;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListDirection, ListItem, ListState, Paragraph},
};

static LIST_STATE: Lazy<ListState> = Lazy::new(|| ListState::default());

pub fn render(app: &mut App, frame: &mut Frame) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(1),
    ]);
    let [help_area, input_area, list_area] = vertical.areas(frame.size());

    let (msg, style) = match app.mode {
        Mode::Normal => (
            vec![
                "Press ".into(),
                "q".bold(),
                " to save & exit, ".into(),
                "i".bold(),
                " to start editing, ".bold(),
                "j".bold(),
                " move selection down, ".into(),
                "k".bold(),
                " move selection up, ".into(),
                "Enter".bold(),
                " to delete the selected item".into(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        Mode::Insert => (
            vec![
                "Press ".into(),
                "Esc".bold(),
                " to go back to normal mode or, ".into(),
                "Enter".bold(),
                " to add a new item.".into(),
            ],
            Style::default(),
        ),
    };
    let text = Text::from(Line::from(msg)).patch_style(style);
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, help_area);

    let input = Paragraph::new(app.input.as_str())
        .style(match app.mode {
            Mode::Normal => Style::default(),
            Mode::Insert => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    frame.render_widget(input, input_area);

    let list_items: Vec<ListItem> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = Line::from(Span::raw(format!("{i}: {m}")));
            ListItem::new(content)
        })
        .collect();

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("Inputs"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    let mut state = LIST_STATE.clone();
    state.select(Some(app.selected_item));

    frame.render_stateful_widget(list, list_area, &mut state);

    match app.mode {
        Mode::Normal => {}
        Mode::Insert => {
            #[allow(clippy::cast_possible_truncation)]
            frame.set_cursor(
                input_area.x + app.cursor_position as u16 + 1,
                input_area.y + 1,
            );
        }
    }
}
