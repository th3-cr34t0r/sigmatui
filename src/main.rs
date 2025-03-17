use sigmatui::TAB_LENGTH;

use std::{cell::RefCell, io, rc::Rc};

mod app;
mod assets;
mod frame;
mod tabs;

use crate::app::App;

use ratzilla::{event::KeyCode, ratatui::Terminal, DomBackend, WebRenderer};

fn main() -> io::Result<()> {
    let app: Rc<RefCell<App>> = Rc::new(RefCell::new(App::new()));
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    terminal.on_key_event({
        let app_cloned = app.clone();
        move |key_event| match key_event.code {
            KeyCode::Left => {
                let mut app = app_cloned.borrow_mut();
                if app.tab_selected != 0 {
                    app.tab_selected -= 1;
                }
            }
            KeyCode::Right => {
                let mut app = app_cloned.borrow_mut();
                if app.tab_selected != (TAB_LENGTH - 1) {
                    app.tab_selected += 1;
                }
            }
            _ => {}
        }
    });

    terminal.draw_web(move |f| {
        let mut app = app.borrow_mut();

        app.run(f);
    });

    Ok(())
}
