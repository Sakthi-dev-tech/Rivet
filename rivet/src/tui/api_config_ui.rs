use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::types::request_type::{ApiMethods, RequestConfig};

pub fn api_config_ui(
    frame: &mut Frame,
    area: Rect,
    current_file: &Option<RequestConfig>,
    current_path: Option<&str>,
    is_hovered: bool,
    is_focused: bool,
) {
    match current_file {
        Some(request_config) => {
            let border_style = if is_hovered {
                Style::default().fg(Color::Blue)
            } else {
                Style::default()
            };

            let method_text: Span = match request_config.method {
                ApiMethods::GET => Span::from(format!(" {:?} ", request_config.method))
                    .black()
                    .on_green()
                    .bold(),
                ApiMethods::POST => Span::from(format!(" {:?} ", request_config.method))
                    .black()
                    .on_yellow()
                    .bold(),
                ApiMethods::PUT => Span::from(format!(" {:?} ", request_config.method))
                    .black()
                    .on_blue()
                    .bold(),
                ApiMethods::PATCH => Span::from(format!(" {:?} ", request_config.method))
                    .black()
                    .on_magenta()
                    .bold(),
                ApiMethods::DELETE => Span::from(format!(" {:?} ", request_config.method))
                    .black()
                    .on_red()
                    .bold(),
                ApiMethods::HEAD => Span::from(format!(" {:?} ", request_config.method))
                    .black()
                    .on_cyan()
                    .bold(),
                ApiMethods::OPTIONS => Span::from(format!(" {:?} ", request_config.method))
                    .black()
                    .on_white()
                    .bold(),
            };

            let block = Block::bordered()
                .border_style(border_style)
                .title_top(
                    Line::from(format!(" {} ", current_path.unwrap_or("Request")))
                        .bold()
                        .left_aligned(),
                )
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

            let input = Paragraph::new(Line::from(vec![
                method_text,
                Span::raw("  "),
                Span::from(format!("{}", request_config.url)).bold(),
            ]))
            .block(Block::bordered().border_set(border::ROUNDED));
            frame.render_widget(input, request_area);

            frame.render_widget(Paragraph::new(Line::default()), spacer_area);

            let body_block = Block::bordered()
                .title_top(Line::from(vec![
                    Span::from(" Headers ").dark_gray(),
                    Span::raw(" "),
                    Span::from(" Query ").dark_gray(),
                    Span::raw(" "),
                    Span::from(" Body ").black().on_blue().bold(),
                    Span::raw(" "),
                    Span::from(" Preview ").dark_gray(),
                ]))
                .border_set(border::PLAIN);

            let body_content = request_config
                .body
                .as_ref()
                .map_or("", |body| body.content.as_str());

            let body = Paragraph::new(body_content).block(body_block);
            frame.render_widget(body, body_area);
        }
        None => frame.render_widget(
            Paragraph::new("No API request selected. Select one from the sidebar.")
                .block(Block::bordered().border_set(border::ROUNDED)),
            area,
        ),
    }
}
