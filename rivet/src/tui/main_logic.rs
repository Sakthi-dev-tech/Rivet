use std::{env, io};

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    symbols::border,
    widgets::Block,
};

use crate::{
    actions::ls_action::{ApiCollectionItem, list_collections_from_path},
    tui::{
        api_config_ui::api_config_ui, help_section_ui::help_section_ui, response_ui::response_ui,
        sidebar_ui::sidebar_ui,
    },
    types::request_type::{ApiMethods, RequestBody, RequestConfig},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Panels {
    Sidebar,
    Config,
    Response,
}

pub struct App {
    // General App States
    run_app: bool,
    collections: Vec<ApiCollectionItem>,
    hover_panel: Panels,

    // App states for config
    selected_api_config_file: Option<RequestConfig>,
}

impl App {
    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.run_app = false;
            return Ok(());
        }

        self.hover_panel = match key_event.code {
            KeyCode::Char('h') => match self.hover_panel {
                Panels::Config => Panels::Sidebar,
                section => section,
            },
            KeyCode::Char('l') => match self.hover_panel {
                Panels::Sidebar => Panels::Config,
                section => section,
            },
            KeyCode::Char('j') => match self.hover_panel {
                Panels::Sidebar | Panels::Config => Panels::Response,
                section => section,
            },
            KeyCode::Char('k') => match self.hover_panel {
                Panels::Response => Panels::Config,
                section => section,
            },
            _ => self.hover_panel,
        };

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

        frame.render_widget(
            sidebar_ui(
                &self.collections,
                self.hover_panel == Panels::Sidebar,
            ),
            sidebar_area,
        );
        api_config_ui(
            frame,
            config_area,
            &self.selected_api_config_file,
            self.hover_panel == Panels::Config,
        );
        frame.render_widget(
            response_ui(self.hover_panel == Panels::Response),
            response_section,
        );
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
        hover_panel: Panels::Sidebar,

        // TODO: Currently using a mock request config
        selected_api_config_file: Some(RequestConfig {
            method: ApiMethods::HEAD,
            url: String::from("www.example.com"),
            params: None,
            auth: None,
            headers: None,
            body: Some(RequestBody {
                content: String::from(
                    r#"{
  "username": "johndoe",
  "email": "john.doe@example.com",
  "age": 28,
  "is_active": true,
  "skills": ["JavaScript", "Python", "SQL"],
  "address": {
    "street": "123 Main Street",
    "city": "Singapore",
    "zipcode": "730000"
  }
}"#,
                ),
            }),
            config: None,
        }),
    };
    app.run(terminal)
}
