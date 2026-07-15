use ratatui::{
    Frame, layout::Rect, style::{Color, Style, Stylize}, symbols::border, text::{Line, Span}, widgets::{Block, List, ListItem, ListState},
};

use crate::{actions::ls_action::ApiCollectionItem, types::request_type::ApiMethods};

/// Gets the prefix String for the folder/request in order to render a clear folder structure
fn tree_prefix(ancestors: &[bool], is_last: bool) -> String {
    let mut prefix = String::new();

    for &ancestor_is_last in ancestors {
        if ancestor_is_last {
            prefix.push_str("   ");
        } else {
            prefix.push_str("│  ");
        }
    }

    if is_last {
        prefix.push_str("└─ ");
    } else {
        prefix.push_str("├─ ");
    }

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

/// Returns a flat list of Ratatui ListItems that contains renderable text
/// for each row in the sidebar.
///
/// This will give a clear folder structure in the sidebar when iterated to form the sidebar widget
fn collection_items<'a>(items: &'a [ApiCollectionItem], ancestors: &[bool]) -> Vec<ListItem<'a>> {
    let mut list_items = Vec::new();

    for (index, item) in items.iter().enumerate() {
        let is_last = index == items.len() - 1;

        // Returns a prefix string for the folder structure
        let prefix = tree_prefix(ancestors, is_last);

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

                    list_items.extend(collection_items(children, &child_ancestors));
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

    list_items
}

pub fn sidebar_ui(
    frame: &mut Frame,
    area: Rect,
    collections: &[ApiCollectionItem],
    is_hovered: bool,
    is_focused: bool,
) {
    let items = collection_items(collections, &[]);
    let mut state = ListState::default();
    state.select(Some(0));

    let border_style = if is_hovered {
        Style::default().fg(Color::Blue)
    } else {
        Style::default()
    };

    let sidebar_widget = List::new(items).block(
        Block::bordered()
            .border_style(border_style)
            .title_top(Line::from(" <●> Rivet APIs ").bold().left_aligned())
            .border_set(if is_focused {
                border::DOUBLE
            } else {
                border::ROUNDED
            }),
    )
        .highlight_style(Style::default()
            .bg(Color::Indexed(237)) 
            .fg(Color::White))
        .highlight_symbol("> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(sidebar_widget, area, &mut state);
}
