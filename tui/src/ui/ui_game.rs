use log::{LevelFilter, info};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, LineGauge, Paragraph},
};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetState};

use crate::data::{AppData, Settings};

pub fn ui_game_render(
    frame: &mut Frame,
    area: Rect,
    app_data: &mut AppData,
    setting: &mut Settings,
) {
    frame.render_widget(app_data.message.as_str(), area);
    let download_state = app_data.download_data.download_pool.query();
    // 给进度条留够足够的高度
    let progressing_task: Vec<_> = download_state
        .per_task
        .iter()
        .filter_map(|(filename, progress, downloaded_size, file_len, speed)| {
            if *progress < 100.0 && *progress > 0.0 {
                Some((
                    filename.clone(),
                    *progress,
                    *downloaded_size,
                    *file_len,
                    *speed,
                ))
            } else {
                None
            }
        })
        .collect();

    let progress_len = if progressing_task.is_empty() {
        0
    } else {
        progressing_task.len() + 3 // 额外的总进度条和block
    };

    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Percentage(100),
            Constraint::Length(progress_len as u16),
        ])
        .split(area);

    let start_block = Block::default()
        .title(" Game Page ")
        .borders(ratatui::widgets::Borders::ALL);
    frame.render_widget(start_block, layout[0]);

    let log_stage = TuiWidgetState::new().set_default_display_level(LevelFilter::Debug);
    log_stage.transition(tui_logger::TuiWidgetEvent::HideKey);
    let logger = TuiLoggerSmartWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Magenta))
        .style_info(Style::default().fg(Color::Cyan))
        .output_separator(':')
        .output_timestamp(Some("%H:%M:%S".to_string()))
        .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
        .output_target(true)
        .output_file(true)
        .output_line(true)
        .state(&log_stage);
    frame.render_widget(logger, layout[1]);

    if progress_len != 0 {
        let process_block = Block::default()
            .title(" Progress ")
            .borders(ratatui::widgets::Borders::ALL);
        let area = process_block.inner(layout[2]);
        frame.render_widget(process_block, layout[2]);

        #[allow(unused_assignments)]
        let mut current_height = 0;
        if !progressing_task.is_empty() {
            for (filename, _progress, downloaded_size, file_len, speed) in progressing_task {
                let current_area = Rect {
                    y: area.y + current_height,
                    ..area
                };

                let progress = downloaded_size as f64 / file_len as f64;
                let label = format!("{}byte/s {:.1}%", speed, progress * 100.0,);
                let chunks = Layout::default()
                    .direction(ratatui::layout::Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(100), // 进度条
                        Constraint::Length(1),
                        Constraint::Length(Line::from(label.clone()).width() as u16), // 百分比文字
                    ])
                    .split(current_area);

                let line_gauge = LineGauge::default()
                    .ratio(progress)
                    .label(filename)
                    .filled_style(Style::default().fg(Color::LightMagenta));
                frame.render_widget(line_gauge, chunks[0]);

                let percentage_paragraph = Paragraph::new(label).style(Style::default().white());
                frame.render_widget(percentage_paragraph, chunks[2]);
                current_height += 1;
            }

            let current_area = Rect {
                y: area.y + current_height,
                ..area
            };
            let progress = download_state.total / download_state.all_total;
            let label = format!("{}byte/s {:.1}%", download_state.speed, progress * 100.0,);
            let chunks = Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(100), // 进度条
                    Constraint::Length(1),
                    Constraint::Length(Line::from(label.clone()).width() as u16), // 百分比文字
                ])
                .split(current_area);

            let line_gauge = LineGauge::default()
                .ratio(progress)
                .label("total:")
                .filled_style(Style::default().fg(Color::Yellow));
            frame.render_widget(line_gauge, chunks[0]);

            let percentage_paragraph = Paragraph::new(label).style(Style::default().white());
            frame.render_widget(percentage_paragraph, chunks[2]);
            current_height += 1;
        }

        if app_data.game_data.is_starting > 0 {
            let current_area = Rect {
                y: area.y + current_height,
                ..area
            };

            let progress = app_data.game_data.is_starting as f64 / 100.0;
            let line_gauge = LineGauge::default()
                .ratio(progress)
                .label("Starting...")
                .filled_style(Style::default().fg(Color::LightGreen));
            frame.render_widget(line_gauge, current_area);
            current_height += 1;
        }
    }
}
