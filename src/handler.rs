use crate::app::{App, AppResult, Mode};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.mode {
        Mode::Normal => match key_event.code {
            KeyCode::Char('i') => {
                app.mode = Mode::Insert;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.move_selection_up();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.move_selection_down();
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                app.toggle_selection_complete();
            }
            KeyCode::Char('d') => {
                app.mode = Mode::Confirmation;
            }
            KeyCode::Char('D') => {
                app.delete_selected();
            }
            KeyCode::Char('q') => {
                app.running = false;
                app.write_items_to_file();
            }
            _ => {}
        },
        Mode::Confirmation => {
            if key_event.code == KeyCode::Char('y') {
                app.delete_selected();
            }
            app.mode = Mode::Normal;
        }
        Mode::Insert if key_event.kind == KeyEventKind::Press => match key_event.code {
            KeyCode::Enter => {
                app.submit_input();
            }
            KeyCode::Char(to_insert) => {
                app.enter_char(to_insert);
            }
            KeyCode::Backspace => {
                app.delete_char();
            }
            KeyCode::Left => {
                app.move_cursor_left();
            }
            KeyCode::Right => {
                app.move_cursor_right();
            }
            KeyCode::Esc => {
                app.mode = Mode::Normal;
            }
            _ => {}
        },
        Mode::Insert => {}
    }
    Ok(())
}
