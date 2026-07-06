use std::io;

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame, layout::{Constraint, Layout}, symbols::border, widgets::Block ,
};

use crate::tui::sidebar_ui::sidebar_ui;

struct App {
    run_app: bool,
}

impl App {
    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.run_app = false;
        }

        Ok(())
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.run_app {
            terminal.draw(|frame| self.draw(frame))?;

            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        let block = Block::bordered()
            .border_set(border::ROUNDED);
        let inner = block.inner(area);

        let [app_ui_area, help_row_area] = Layout::vertical([
            Constraint::Percentage(90),
            Constraint::Percentage(10)
        ])
            .areas(inner);

        let [sidebar_area, config_area] = Layout::horizontal([
            Constraint::Percentage(10),
            Constraint::Percentage(90)
        ])
            .areas(app_ui_area);

        frame.render_widget(block, area);
        frame.render_widget(sidebar_ui(), sidebar_area);
    }
}

pub fn tui_app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App { run_app: true };
    app.run(terminal)
}
