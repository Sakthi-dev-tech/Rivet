use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, ListState},
};

pub fn sidebar_ui(
    frame: &mut Frame,
    area: Rect,
    collection_items: &[ListItem<'static>],
    sidebar_state: &mut ListState,
    is_hovered: bool,
    is_focused: bool,
) {
    let border_style = if is_hovered {
        Style::default().fg(Color::Blue)
    } else {
        Style::default()
    };

    let border_set = if is_focused {
        border::DOUBLE
    } else {
        border::ROUNDED
    };

    let sidebar_widget = List::new(collection_items.iter().cloned())
        .block(
            Block::bordered()
                .border_style(border_style)
                .title_top(Line::from(" <●> Rivet APIs ").bold().left_aligned())
                .border_set(border_set),
        )
        .highlight_style(Style::default().bg(Color::Indexed(237)).fg(Color::White))
        .highlight_symbol("> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(sidebar_widget, area, sidebar_state);
}
