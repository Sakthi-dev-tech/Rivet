use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::types::request_type::RequestConfig;

pub fn api_config_ui(
    frame: &mut Frame,
    area: Rect,
    _current_file: &Option<RequestConfig>,
    is_focused: bool,
) {
    match _current_file {
        Some(request_config) => {
            let border_style = if is_focused {
                Style::default().fg(Color::Blue)
            } else {
                Style::default()
            };

            let block = Block::bordered()
                .border_style(border_style)
                .title_top(Line::from(" users/create-user ").bold().left_aligned())
                .title_top(Line::from(" Env: .env ").blue().right_aligned())
                .title_top(Line::from(" [saved] ").green().right_aligned())
                .border_set(if is_focused {
                    border::DOUBLE
                } else {
                    border::ROUNDED
                });
            let inner = block.inner(area);

            frame.render_widget(block, area);

            let [request_area, spacer_area, body_area] = Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Min(5),
            ])
            .areas(inner);

            let [input_area, config_area] =
                Layout::horizontal([Constraint::Min(24), Constraint::Length(34)])
                    .areas(request_area);

            let [_, tabs_area, _] = Layout::vertical([
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ])
            .areas(config_area);

            let input = Paragraph::new(Line::from(vec![
                Span::from(
                    format!(" {} ", request_config.method))
                .black()
                .on_yellow()
                .bold(),
                Span::raw("  "),
                Span::from(format!("{}", request_config.url)).bold(),
            ]))
            .block(Block::bordered().border_set(border::ROUNDED));
            frame.render_widget(input, input_area);

            let tabs = Paragraph::new(Line::from(vec![
                Span::from(" Headers ").dark_gray(),
                Span::raw(" "),
                Span::from(" Query ").dark_gray(),
                Span::raw(" "),
                Span::from(" Body ").white().on_blue().bold(),
                Span::raw(" "),
                Span::from(" Preview ").dark_gray(),
            ]))
            .right_aligned();
            frame.render_widget(tabs, tabs_area);

            frame.render_widget(Paragraph::new(Line::default()), spacer_area);

            let body_block = Block::bordered()
                .title_top(Line::from(" Body ").bold())
                .border_set(border::PLAIN);

            let body_content = request_config
                .body
                .as_ref()
                .map_or("", |body| body.content.as_str());

            let body = Paragraph::new(body_content).block(body_block);
            frame.render_widget(body, body_area);
        }
        None => {}
    }
}
