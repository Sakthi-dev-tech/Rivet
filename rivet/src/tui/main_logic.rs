use std::io;

use ratatui::{DefaultTerminal, Frame};

pub fn tui_app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render (frame: &mut Frame) {
    frame.render_widget("Hello world!", frame.area());
}
