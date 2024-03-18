use argh::FromArgs;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use todo::app::{App, AppResult};
use todo::event::{Event, EventHandler};
use todo::handler::handle_key_events;
use todo::tui::Tui;

#[derive(FromArgs)]
/// A suckless todo app.
struct AppArgs {}

fn main() -> AppResult<()> {
    let _: AppArgs = argh::from_env();
    let mut app = App::new();
    app.list.items = app.file.read_items_from_file()?;

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
