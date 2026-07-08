use ratatui::{
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, List, ListItem, Widget},
};

use crate::actions::ls_action::ApiCollectionItem;

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

fn collection_items<'a>(items: &'a [ApiCollectionItem], ancestors: &[bool]) -> Vec<ListItem<'a>> {
    let mut list_items = Vec::new();

    for (index, item) in items.iter().enumerate() {
        let is_last = index == items.len() - 1;
        let prefix = tree_prefix(ancestors, is_last);

        match item {
            ApiCollectionItem::Folder { name, children } => {
                list_items.push(ListItem::new(
                    Line::from(format!("{prefix}\u{f07b} {name}")).bold(),
                ));

                let mut child_ancestors = ancestors.to_vec();
                child_ancestors.push(is_last);

                list_items.extend(collection_items(children, &child_ancestors));
            }

            ApiCollectionItem::Request { name, method, path } => {
                let method_span = match method.as_str() {
                    "GET" => Span::from(format!(" {method} ")).black().on_green(),
                    "POST" => Span::from(format!(" {method} ")).black().on_yellow(),
                    "PUT" => Span::from(format!(" {method} ")).black().on_blue(),
                    "PATCH" => Span::from(format!(" {method} ")).black().on_magenta(),
                    "DELETE" => Span::from(format!(" {method} ")).white().on_red(),
                    _ => Span::from(format!(" {method} ")).white().on_dark_gray(),
                };

                list_items.push(ListItem::new(Line::from(vec![
                    Span::raw(prefix),
                    method_span,
                    Span::raw(format!(" {name} {path}")),
                ])));
            }
        }
    }

    list_items
}

pub fn sidebar_ui(collections: &[ApiCollectionItem]) -> impl Widget {
    let items = collection_items(collections, &[]);

    List::new(items).block(
        Block::bordered()
            .title_top(Line::from(" Your APIs ").bold().left_aligned())
            .border_set(border::ROUNDED),
    )
}
