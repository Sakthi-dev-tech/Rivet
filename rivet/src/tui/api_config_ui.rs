use ratatui::{
    buffer::Buffer, layout::{Constraint, Layout, Rect}, style::Stylize, symbols::border, text::{Line, Span}, widgets::{Block, Paragraph, Widget},
};

struct ApiConfigUi;

impl Widget for ApiConfigUi {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title_top(Line::from(" users/create-user ").bold().left_aligned())
            .title_top(Line::from(" [saved] ").green().right_aligned())
            .border_set(border::ROUNDED);
        let inner = block.inner(area);

        block.render(area, buf);

        let [request_area, spacer_area, body_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(5),
        ])
        .areas(inner);

        let [input_area, tabs_area] =
            Layout::horizontal([Constraint::Min(24), Constraint::Length(34)]).areas(request_area);

        Paragraph::new(Line::from(vec![
            Span::from(" POST ").black().on_yellow().bold(),
            Span::raw("  "),
            Span::from("some_url/users").bold(),
        ]))
        .block(Block::bordered()
            .border_set(border::ROUNDED))
        .render(input_area, buf);

        Paragraph::new(Line::from(vec![
            Span::from(" Headers ").dark_gray(),
            Span::raw(" "),
            Span::from(" Query ").dark_gray(),
            Span::raw(" "),
            Span::from(" Body ").white().on_blue().bold(),
            Span::raw(" "),
            Span::from(" Preview ").dark_gray(),
        ]))
        .right_aligned()
        .render(tabs_area, buf);

        Paragraph::new(Line::default()).render(spacer_area, buf);

        let body_block = Block::bordered()
            .title_top(Line::from(" Body ").bold())
            .border_set(border::PLAIN);

        Paragraph::new(vec![
            Line::from("{").dark_gray(),
            Line::from(vec![
                Span::raw("  \"name\": "),
                Span::from("\"{{USER_NAME}}\"").yellow(),
                Span::raw(","),
            ]),
            Line::from(vec![
                Span::raw("  \"email\": "),
                Span::from("\"{{USER_EMAIL}}\"").yellow(),
            ]),
            Line::from("}").dark_gray(),
        ])
        .block(body_block)
        .render(body_area, buf);
    }
}

pub fn api_config_ui() -> impl Widget {
    ApiConfigUi
}
