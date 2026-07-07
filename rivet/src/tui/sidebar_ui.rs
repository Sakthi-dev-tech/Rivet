use ratatui::{
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, Widget},
};

use crate::actions::ls_action::ApiCollectionItem;

fn collection_items<'a>(items: &[ApiCollectionItem], depth: usize) -> Vec<ListItem<'a>> {
    let mut list_items = Vec::new();

    for item in items {
        let indent = " ".repeat(depth);

        match item {
            ApiCollectionItem::Folder { name, children } => {
                list_items.push(ListItem::new(
                    Line::from(format!("{indent}\u{f07b} {name}")).bold(),
                ));

                list_items.extend(collection_items(children, depth + 1));
            }

            ApiCollectionItem::Request { name, method, path } => {
                list_items.push(ListItem::new(Line::from(format!(
                    "{indent}{method} {name} {path}"
                ))));
            }
        }
    }

    list_items
}

pub fn sidebar_ui(collections: &[ApiCollectionItem]) -> impl Widget {
    let items = collection_items(collections, 0);

    List::new(items).block(
        Block::bordered()
            .title_top(Line::from(" Your APIs ").bold().left_aligned())
            .border_set(border::ROUNDED),
    )
}
