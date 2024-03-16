use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use todo::app::{App, AppResult};
use todo::event::{Event, EventHandler};
use todo::handler::handle_key_events;
use todo::tui::Tui;

fn main() -> AppResult<()> {
    let mut app = App::new();

    app.read_items_from_file();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
