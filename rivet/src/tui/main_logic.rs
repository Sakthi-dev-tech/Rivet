use std::{env, io};

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    symbols::border,
    widgets::{Block, ListState},
};

use crate::{
    actions::ls_action::{ApiCollectionItem, list_collections_from_path},
    tui::{
        api_config_ui::api_config_ui, help_section_ui::help_section_ui, response_ui::response_ui,
        sidebar_ui::sidebar_ui,
    },
    types::request_type::{ApiMethods, Config, RequestBody, RequestConfig},
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
    hovered_panel: Panels,
    is_panel_focused: bool,

    // App state for sidebar
    sidebar_state: ListState,

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

        match key_event.code {
            // When the Enter key is recorded
            KeyCode::Enter => {
                // Before changing the is focused flag to true, we check if it is already true
                // which means that we are already in a panel and we have
                // to do panel-specific control
                #[allow(unused)] // REMOVE LATER
                if (self.is_panel_focused == true) {
                    // TODO: Enter when a panel is focused
                    // for e.g. sidebar should toggle folder or set the selected request file
                } else {
                    // Change the is panel focused flag to true
                    self.is_panel_focused = true;
                }

                return Ok(());
            }
            KeyCode::Esc => {
                self.is_panel_focused = false;
                return Ok(());
            }
            _ => {}
        }

        // Change hovered panel through movement keys if no panel is focused
        self.hovered_panel = {
            match self.is_panel_focused {
                false => match key_event.code {
                    KeyCode::Char('h') => match self.hovered_panel {
                        Panels::Config => Panels::Sidebar,
                        section => section,
                    },
                    KeyCode::Char('l') => match self.hovered_panel {
                        Panels::Sidebar => Panels::Config,
                        section => section,
                    },
                    KeyCode::Char('j') => match self.hovered_panel {
                        Panels::Sidebar | Panels::Config => Panels::Response,
                        section => section,
                    },
                    KeyCode::Char('k') => match self.hovered_panel {
                        Panels::Response => Panels::Config,
                        section => section,
                    },
                    _ => self.hovered_panel,
                },
                true => self.hovered_panel,
            }
        };

        // If a panel is focused, we run the required function block
        // for the hovered + focused panel
        if self.is_panel_focused {
            match self.hovered_panel {
                // If focused panel is the sidebar
                Panels::Sidebar => match key_event.code {
                    // j and k keys go up and down the list
                    KeyCode::Char('j') => {
                        if let Some(curr_idx) = self.sidebar_state.selected() {
                            self.sidebar_state.select(Some(curr_idx + 1));
                        } else {
                            self.sidebar_state.select(Some(0));
                        }
                    },
                    KeyCode::Char('k') => {
                        if let Some(curr_idx) = self.sidebar_state.selected() {
                            self.sidebar_state.select(Some(curr_idx.saturating_sub(1)));
                        }  
                    }
                    _ => {}
                },
                _ => {}
            }
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

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let sidebar_is_hovered = self.hovered_panel == Panels::Sidebar;
        let config_is_hovered = self.hovered_panel == Panels::Config;
        let response_is_hovered = self.hovered_panel == Panels::Response;

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

        sidebar_ui(
            frame,
            sidebar_area,
            &self.collections,
            &mut self.sidebar_state,
            sidebar_is_hovered,
            sidebar_is_hovered && self.is_panel_focused,
        );

        api_config_ui(
            frame,
            config_area,
            &self.selected_api_config_file,
            config_is_hovered,
            config_is_hovered && self.is_panel_focused,
        );

        response_ui(
            frame,
            response_section,
            response_is_hovered,
            response_is_hovered && self.is_panel_focused,
        );

        help_section_ui(frame, help_section);
    }
}

pub fn tui_app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let current_path = env::current_dir()?;
    let collection_path = current_path.join(".rivet/collections");
    let collections = list_collections_from_path(&collection_path)?;

    let mut sidebar_state = ListState::default();
    sidebar_state.select(Some(0));

    let mut app = App {
        run_app: true,
        collections,
        hovered_panel: Panels::Sidebar,
        is_panel_focused: false,

        sidebar_state,

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
            config: Some(Config { timeout: 30 }),
        }),
    };
    app.run(terminal)
}
