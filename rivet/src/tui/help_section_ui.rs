use ratatui::{
    Frame, layout::Rect, widgets::Block, symbols::border,
};

pub fn help_section_ui(
    frame: &mut Frame,
    area: Rect
    ) {
    let help_widget = Block::bordered()
        .border_set(border::ROUNDED)
        .title_top(" Help ");

    frame.render_widget(help_widget, area);
}
