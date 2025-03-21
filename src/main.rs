use std::{io, rc::Rc};

mod app;
mod assets;
mod frame;
mod tabs;

use crate::app::App;

use ratzilla::{ratatui::Terminal, DomBackend, WebRenderer};

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let state: Rc<App> = Rc::new(App::default());

    let event_state = Rc::clone(&state);
    terminal.on_key_event({
        move |key_event| {
            event_state.handle_events(key_event);
        }
    });

    let render_state = Rc::clone(&state);
    terminal.draw_web(move |frame| {
        render_state.render(frame);
    });

    Ok(())
}
