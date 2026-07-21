use std::{env, fs, io, path::PathBuf};

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, ListItem, ListState},
};

use crate::{
    actions::ls_action::{ApiCollectionItem, list_collections_from_path},
    tui::{
        api_config_ui::api_config_ui, help_section_ui::help_section_ui, response_ui::response_ui,
        sidebar_ui::sidebar_ui,
    },
    types::request_type::{ApiMethods, RequestConfig},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Panels {
    Sidebar,
    Config,
    Response,
}

/*
 * =================================================================
 * FUNCTIONS FOR SIDEBAR LOGIC
 * =================================================================
 */
/// Gets the prefix String for the folder/request in order to render a clear folder structure
fn tree_prefix(ancestors: &[bool], is_last: bool) -> String {
    let mut prefix = String::new();

    for &ancestor_is_last in ancestors {
        prefix.push_str(if ancestor_is_last { "   " } else { "│  " });
    }

    prefix.push_str(if is_last { "└─ " } else { "├─ " });
    prefix
}

fn method_span(method: Option<ApiMethods>) -> Span<'static> {
    match method {
        Some(ApiMethods::GET) => Span::from(" GET ").black().on_green(),
        Some(ApiMethods::POST) => Span::from(" POST ").black().on_yellow(),
        Some(ApiMethods::PUT) => Span::from(" PUT ").black().on_blue(),
        Some(ApiMethods::PATCH) => Span::from(" PATCH ").black().on_magenta(),
        Some(ApiMethods::DELETE) => Span::from(" DELETE ").black().on_red(),
        Some(ApiMethods::HEAD) => Span::from(" HEAD ").black().on_cyan(),
        Some(ApiMethods::OPTIONS) => Span::from(" OPTIONS ").black().on_white(),
        None => Span::from(" Unknown ").black().on_dark_gray(),
    }
}

/// Flattens the collection tree into owned, renderable sidebar rows and matching paths.
fn collection_items(
    items: &[ApiCollectionItem],
    ancestors: &[bool],
    parent_path: &[usize],
) -> (Vec<ListItem<'static>>, Vec<SidebarRow>) {
    let mut list_items = Vec::new();
    let mut sidebar_rows = Vec::new();

    for (index, item) in items.iter().enumerate() {
        let is_last = index == items.len() - 1;
        let prefix = tree_prefix(ancestors, is_last);
        let mut item_path = parent_path.to_vec();
        item_path.push(index);

        sidebar_rows.push(SidebarRow {
            path: item_path.clone(),
        });

        match item {
            ApiCollectionItem::Folder {
                name,
                children,
                is_expanded,
            } => {
                let icon = if *is_expanded { "\u{f07c}" } else { "\u{f07b}" };
                list_items.push(ListItem::new(
                    Line::from(format!("{prefix}{icon} {name}")).yellow().bold(),
                ));

                if *is_expanded {
                    let mut child_ancestors = ancestors.to_vec();
                    child_ancestors.push(is_last);
                    let (child_items, child_rows) =
                        collection_items(children, &child_ancestors, &item_path);
                    list_items.extend(child_items);
                    sidebar_rows.extend(child_rows);
                }
            }
            ApiCollectionItem::Request { name, method, path } => {
                list_items.push(ListItem::new(Line::from(vec![
                    Span::raw(prefix),
                    method_span(*method),
                    Span::raw(format!(" {name} {path}")),
                ])));
            }
        }
    }

    (list_items, sidebar_rows)
}

/// This function handles Enter key being pressed in the sidebar
fn sidebar_ui_handle_enter(app: &mut App) {
    let Some(entered_idx) = app.sidebar_state.selected() else {
        return;
    };

    let Some(row) = app.sidebar_rows.get(entered_idx) else {
        return;
    };

    let Some((&selected_idx, parent_indices)) = row.path.split_last() else {
        return;
    };

    let mut items = &mut app.collections;

    for &index in parent_indices {
        let Some(ApiCollectionItem::Folder { children, .. }) = items.get_mut(index) else {
            return;
        };

        items = children;
    }

    let Some(chosen_item) = items.get_mut(selected_idx) else {
        return;
    };

    match chosen_item {
        ApiCollectionItem::Folder { is_expanded, .. } => {
            *is_expanded = !*is_expanded;
            app.refresh_collection_items();
        }

        ApiCollectionItem::Request { path, .. } => {
            let request_path = app.collection_path.join(format!("{path}.toml"));

            let Ok(request_str) = fs::read_to_string(request_path) else {
                panic!("Cannot read selected file!")
            };

            let request_config: RequestConfig = toml::from_str(&request_str).unwrap_or_else(|_| {
                panic!("Format of file is wrong! Follow the given format for the TOML file!")
            });

            app.selected_api_config_path = Some(path.clone());
            app.selected_api_config_file = Some(request_config);
        }
    }
}

struct SidebarRow {
    /// Indices from the root collection slice through each folder's `children` slice.
    path: Vec<usize>,
}

pub struct App {
    // General App States
    run_app: bool,
    collections: Vec<ApiCollectionItem>,
    collection_path: PathBuf,
    hovered_panel: Panels,
    is_panel_focused: bool,

    // App state for sidebar
    collection_items: Vec<ListItem<'static>>,
    sidebar_rows: Vec<SidebarRow>, // Index of row => path to the file/folder
    sidebar_state: ListState,

    // App states for config
    selected_api_config_path: Option<String>,
    selected_api_config_file: Option<RequestConfig>,
}

impl App {
    fn new(collection_path: PathBuf, collections: Vec<ApiCollectionItem>) -> Self {
        let mut app = Self {
            run_app: true,
            collections,
            collection_path,
            hovered_panel: Panels::Sidebar,
            is_panel_focused: false,
            collection_items: Vec::new(),
            sidebar_rows: Vec::new(),
            sidebar_state: ListState::default(),
            selected_api_config_path: None,
            selected_api_config_file: None,
        };
        app.refresh_collection_items();
        app
    }

    fn refresh_collection_items(&mut self) {
        let (collection_items, sidebar_rows) = collection_items(&self.collections, &[], &[]);
        debug_assert_eq!(collection_items.len(), sidebar_rows.len());
        debug_assert!(sidebar_rows.iter().all(|row| !row.path.is_empty()));
        let row_count = collection_items.len();

        self.collection_items = collection_items;
        self.sidebar_rows = sidebar_rows;
        self.sidebar_state.select(if row_count == 0 {
            None
        } else {
            Some(
                self.sidebar_state
                    .selected()
                    .unwrap_or(0)
                    .min(row_count - 1),
            )
        });
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind != KeyEventKind::Press {
            return Ok(());
        }
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.run_app = false;
            return Ok(());
        }

        match key_event.code {
            // When the Enter key is recorded and there is no focused panel
            KeyCode::Enter if !self.is_panel_focused => {
                self.is_panel_focused = true;
                return Ok(());
            }
            KeyCode::Esc => {
                self.is_panel_focused = false;
                return Ok(());
            }
            _ => {}
        }

        // Change hovered panel through movement keys if no panel is focused
        if !self.is_panel_focused {
            self.hovered_panel = match key_event.code {
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
            };
        }

        // If a panel is focused, we run the required function block
        // for the hovered + focused panel
        if self.is_panel_focused {
            match self.hovered_panel {
                // If focused panel is the sidebar
                Panels::Sidebar => match key_event.code {
                    // j and k keys go up and down the list
                    KeyCode::Char('j') => {
                        if let Some(last_idx) = self.collection_items.len().checked_sub(1) {
                            let next_idx = self
                                .sidebar_state
                                .selected()
                                .map_or(0, |idx| idx.saturating_add(1).min(last_idx));
                            self.sidebar_state.select(Some(next_idx));
                        }
                    }
                    KeyCode::Char('k') => {
                        if !self.collection_items.is_empty() {
                            let previous_idx = self
                                .sidebar_state
                                .selected()
                                .map_or(0, |idx| idx.saturating_sub(1));
                            self.sidebar_state.select(Some(previous_idx));
                        }
                    }
                    KeyCode::Enter => {
                        sidebar_ui_handle_enter(self);
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
            &self.collection_items,
            &mut self.sidebar_state,
            sidebar_is_hovered,
            sidebar_is_hovered && self.is_panel_focused,
        );

        api_config_ui(
            frame,
            config_area,
            &self.selected_api_config_file,
            self.selected_api_config_path.as_deref(),
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

    App::new(collection_path, collections).run(terminal)
}
