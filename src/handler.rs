use crate::app::{App, AppResult};
use crate::file::FileMode;
use crate::input::InputMode;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.input.mode {
        InputMode::Normal => match key_event.code {
            KeyCode::Char('i') => {
                app.list.clear_selection();
                app.input.reset_cursor();
                app.input.mode = InputMode::Insert;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.list.move_selection_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.list.move_selection_down();
            }
            KeyCode::Enter | KeyCode::Tab | KeyCode::Char(' ') => {
                app.list.toggle_selected_item();
            }
            KeyCode::Char('e') => {
                app.input.set_content(app.list.get_selected_item().unwrap().content.clone());
                app.input.mode = InputMode::Insert;
            }
            KeyCode::Char('d') => {
                app.input.mode = InputMode::Remove;
            }
            KeyCode::Char('G') => {
                app.file.write_items_to_file(&app.list.items)?;
                app.list.clear_selection();
                app.list.selected = true;
                app.file.mode = match app.file.mode {
                    FileMode::Local => FileMode::Global,
                    FileMode::Global => FileMode::Local,
                };
                app.list.items = app.file.read_items_from_file()?;
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                app.file.write_items_to_file(&app.list.items)?;
                app.running = false;
            }
            _ => {}
        },
        InputMode::Insert if key_event.kind == KeyEventKind::Press => match key_event.code {
            KeyCode::Enter => {
                let input_string = app.input.input.trim().to_string();
                if input_string.is_empty() {
                    return Ok(());
                }
                if app.list.selected {
                    app.list.edit_selected_item(input_string);
                } else {
                    app.list.add_item(input_string);
                }
                app.input.clear();
                app.input.reset_cursor();
                app.input.mode = InputMode::Normal;
            }
            KeyCode::Char(to_insert) => {
                app.input.enter_char(to_insert);
            }
            KeyCode::Backspace => {
                app.input.delete_char();
            }
            KeyCode::Left => {
                app.input.move_cursor_left();
            }
            KeyCode::Right => {
                app.input.move_cursor_right();
            }
            KeyCode::Esc => {
                app.input.input = match app.list.selected {
                    true => "".to_string(),
                    false => app.input.input.clone(),
                };
                app.input.mode = InputMode::Normal;
                app.list.selected = true;
            }
            _ => {}
        },
        InputMode::Insert => {}
        InputMode::Remove if key_event.kind == KeyEventKind::Press => match key_event.code {
            KeyCode::Char('y') => {
                app.list.remove_selected_item();
                app.input.mode = InputMode::Normal;
            }
            KeyCode::Char('n') => {
                app.input.mode = InputMode::Normal;
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
