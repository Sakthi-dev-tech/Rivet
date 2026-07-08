use ratatui::{
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, List, ListItem, Widget},
};

use crate::{actions::ls_action::ApiCollectionItem, types::request_type::ApiMethods};

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

fn collection_items<'a>(items: &'a [ApiCollectionItem], ancestors: &[bool]) -> Vec<ListItem<'a>> {
    let mut list_items = Vec::new();

    for (index, item) in items.iter().enumerate() {
        let is_last = index == items.len() - 1;
        let prefix = tree_prefix(ancestors, is_last);

        match item {
            ApiCollectionItem::Folder { name, children } => {
                list_items.push(ListItem::new(
                    Line::from(format!("{prefix}\u{f07c} {name}")).yellow().bold(),
                ));

                let mut child_ancestors = ancestors.to_vec();
                child_ancestors.push(is_last);

                list_items.extend(collection_items(children, &child_ancestors));
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

pub fn sidebar_ui(collections: &[ApiCollectionItem], is_focused: bool) -> impl Widget {
    let items = collection_items(collections, &[]);
    let border_style = if is_focused {
        Style::default().fg(Color::Blue)
    } else {
        Style::default()
    };

    List::new(items).block(
        Block::bordered()
            .border_style(border_style)
            .title_top(Line::from(" <●> Rivet APIs ").bold().left_aligned())
            .border_set(if is_focused {
                border::DOUBLE
            } else {
                border::ROUNDED
            }),
    )
}
