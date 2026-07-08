use std::{env, io};

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    symbols::border,
    widgets::Block,
};

use crate::{
    actions::ls_action::{ApiCollectionItem, list_collections_from_path}, tui::{api_config_ui::api_config_ui, help_section_ui::help_section_ui, response_ui::response_ui, sidebar_ui::sidebar_ui},
};

struct App {
    run_app: bool,
    collections: Vec<ApiCollectionItem>,
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

        let block = Block::bordered().border_set(border::EMPTY);
        let inner = block.inner(area);

        let [api_section, response_section, help_section] = Layout::vertical([
            Constraint::Percentage(48),
            Constraint::Percentage(42),
            Constraint::Percentage(10),
        ])
        .areas(inner);

        let [sidebar_area, config_area] =
            Layout::horizontal([Constraint::Percentage(15), Constraint::Percentage(85)])
                .areas(api_section);

        frame.render_widget(block, area);

        frame.render_widget(sidebar_ui(&self.collections), sidebar_area);
        frame.render_widget(api_config_ui(), config_area);
        frame.render_widget(response_ui(), response_section);
        frame.render_widget(help_section_ui(), help_section);
    }
}

pub fn tui_app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let current_path = env::current_dir()?;
    let collection_path = current_path.join(".rivet/collections");
    let collections = list_collections_from_path(&collection_path)?;

    let mut app = App {
        run_app: true,
        collections,
    };
    app.run(terminal)
}
