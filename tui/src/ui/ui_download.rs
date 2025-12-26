use std::time::Instant;

use crossterm::event::KeyCode;
use log::info;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{LineGauge, Paragraph, Tabs},
};
use rust_i18n::t;
use tui_input::{Input, backend::crossterm::EventHandler};

use crate::{
    api::is_contains_rect,
    data::{AppData, download_data::DebugFocus},
};

pub fn ui_download_render(frame: &mut Frame, area: Rect, app_data: &mut AppData) {
    let download_state = app_data.download_data.download_pool.query();

    let mut layout_item = vec![Constraint::Length(1), Constraint::Percentage(100)];
    if download_state.all_total > 0.0 {
        layout_item.push(Constraint::Length(1));
    }

    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(layout_item)
        .split(area);

    // tabs
    let mut tabs_item = vec![t!("ui.download.minecraft"), t!("ui.download.modpack")];
    if cfg!(debug_assertions) {
        tabs_item.push(t!("ui.download.debug"));
    }

    let tabs_item_str_len = tabs_item
        .iter()
        .map(|f| Line::from(f.clone()).width())
        .collect::<Vec<_>>();

    if let Some(key_event) = &app_data.key_event
        && key_event.is_press()
        && !app_data.download_data.is_input_mode
        && app_data
            .download_data
            .tabs_keys_instant
            .elapsed()
            .as_millis()
            > 200
    {
        app_data.download_data.tabs_keys_instant = Instant::now();
        match key_event.code {
            KeyCode::Left => {
                if app_data.download_data.tabs_list_state > 0 {
                    app_data
                        .download_data
                        .change_tabs_list_state(app_data.download_data.tabs_list_state - 1);
                }
            }
            KeyCode::Right => {
                if app_data.download_data.tabs_list_state < tabs_item.len() - 1 {
                    app_data
                        .download_data
                        .change_tabs_list_state(app_data.download_data.tabs_list_state + 1);
                }
            }
            _ => {}
        }
    }
    if let Some(mouse_event) = &app_data.mouse_event
        && mouse_event.kind
            == crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left)
    {
        if let Some((x, y)) = is_contains_rect(mouse_event.column, mouse_event.row, &area) {
            if y == 0 {
                // tabs is first line in download area
                let mut current_len = 0;
                for (i, len) in tabs_item_str_len.iter().enumerate() {
                    if x < *len as u16 + current_len + 2 {
                        // 2 is padding
                        app_data.download_data.change_tabs_list_state(i);
                        break;
                    }
                    current_len += *len as u16 + 3; // 3 is padding and divider
                }
            }
        }
    }

    let tab = Tabs::new(tabs_item)
        .style(Style::default().white())
        .highlight_style(Style::default().yellow())
        .select(app_data.download_data.tabs_list_state);

    frame.render_widget(tab, layout[0]);

    match app_data.download_data.tabs_list_state {
        2 => {
            #[cfg(debug_assertions)]
            {
                debug_render(frame, layout[1], app_data);
            }
        }
        _ => {}
    }

    if download_state.all_total > 0.0 {
        let progress = download_state.total / download_state.all_total;
        let label = format!("{}byte/s {:.1}%", download_state.speed, progress * 100.0,);
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Percentage(100), // 进度条
                Constraint::Length(1),
                Constraint::Length(Line::from(label.clone()).width() as u16), // 百分比文字
            ])
            .split(layout[2]);

        let line_gauge = LineGauge::default()
            .ratio(progress)
            .label("")
            .filled_style(Style::default().fg(Color::Yellow));
        frame.render_widget(line_gauge, chunks[0]);

        let percentage_paragraph = Paragraph::new(label).style(Style::default().white());
        frame.render_widget(percentage_paragraph, chunks[2]);
    }
}

fn debug_render(frame: &mut Frame, area: Rect, app_data: &mut AppData) {
    let mut enter_facous = DebugFocus::None;

    if matches!(
        app_data.download_data.debug_focus,
        DebugFocus::InputUrl | DebugFocus::InputPath
    ) {
        app_data.download_data.is_input_mode = true;
    } else {
        app_data.download_data.is_input_mode = false;
    }

    if let Some(key_event) = &app_data.key_event
        && key_event.is_press()
        && app_data
            .download_data
            .debug_keys_instant
            .elapsed()
            .as_millis()
            > 200
    {
        app_data.download_data.debug_keys_instant = Instant::now();
        match key_event.code {
            KeyCode::Tab => {
                app_data.download_data.debug_focus = app_data.download_data.debug_focus.tab();
            }
            KeyCode::Enter
                if matches!(
                    app_data.download_data.debug_focus,
                    DebugFocus::SelectPath | DebugFocus::Download
                ) =>
            {
                enter_facous = app_data.download_data.debug_focus.clone();
            }
            _ => {}
        }
    }

    if let Some(event) = &app_data.event
        && app_data.is_read_event
    {
        if app_data.download_data.debug_focus == DebugFocus::InputUrl {
            app_data.download_data.debug_input_url.handle_event(event);
        } else if app_data.download_data.debug_focus == DebugFocus::InputPath {
            app_data.download_data.debug_input_path.handle_event(event);
        }
    }
    let layout_vertical = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(3)])
        .split(area);

    // url input render
    if let Some(mouse_event) = &app_data.mouse_event
        && mouse_event.kind
            == crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left)
        && is_contains_rect(mouse_event.column, mouse_event.row, &layout_vertical[0]).is_some()
    {
        app_data.download_data.debug_focus = DebugFocus::InputUrl;
    }

    let url_input = Paragraph::new(app_data.download_data.debug_input_url.value())
        .style(Style::default())
        .block(
            ratatui::widgets::Block::default()
                .title(t!("ui.download.debug_tab.url_input"))
                .borders(ratatui::widgets::Borders::ALL)
                .style(
                    if app_data.download_data.debug_focus == DebugFocus::InputUrl {
                        Style::default().yellow()
                    } else {
                        Style::default()
                    },
                ),
        );
    frame.render_widget(url_input, layout_vertical[0]);
    if app_data.download_data.debug_focus == DebugFocus::InputUrl {
        let width = layout_vertical[0].width.max(3) - 3;
        let scroll = app_data
            .download_data
            .debug_input_url
            .visual_scroll(width as usize);
        let x = app_data
            .download_data
            .debug_input_url
            .visual_cursor()
            .max(scroll)
            - scroll
            + 1;
        frame.set_cursor_position((layout_vertical[0].x + x as u16, layout_vertical[0].y + 1))
    }

    let download_lable = t!("ui.download.debug_tab.download_button",);
    let layout_2 = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage(100),
            Constraint::Length(5),
            Constraint::Length(Line::from(download_lable.clone()).width() as u16 + 2),
        ])
        .split(layout_vertical[1]);

    // path input render
    if let Some(mouse_event) = &app_data.mouse_event
        && mouse_event.kind
            == crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left)
        && is_contains_rect(mouse_event.column, mouse_event.row, &layout_2[0]).is_some()
    {
        app_data.download_data.debug_focus = DebugFocus::InputPath;
    }

    let path_input = Paragraph::new(app_data.download_data.debug_input_path.value())
        .style(Style::default())
        .block(
            ratatui::widgets::Block::default()
                .title(t!("ui.download.debug_tab.path_input"))
                .borders(ratatui::widgets::Borders::ALL)
                .style(
                    if app_data.download_data.debug_focus == DebugFocus::InputPath {
                        Style::default().yellow()
                    } else {
                        Style::default()
                    },
                ),
        );
    frame.render_widget(path_input, layout_2[0]);
    if app_data.download_data.debug_focus == DebugFocus::InputPath {
        let width = layout_2[0].width.max(3) - 3;
        let scroll = app_data
            .download_data
            .debug_input_path
            .visual_scroll(width as usize);
        let x = app_data
            .download_data
            .debug_input_path
            .visual_cursor()
            .max(scroll)
            - scroll
            + 1;
        frame.set_cursor_position((layout_2[0].x + x as u16, layout_2[0].y + 1))
    }

    // ... button render
    if let Some(mouse_event) = &app_data.mouse_event
        && mouse_event.kind
            == crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left)
        && is_contains_rect(mouse_event.column, mouse_event.row, &layout_2[1]).is_some()
        && app_data.download_data.debug_enter_instant.elapsed().as_millis() > 200
    {
        app_data.download_data.debug_focus = DebugFocus::SelectPath;
        enter_facous = DebugFocus::SelectPath;
        app_data.download_data.debug_enter_instant = Instant::now();
    }
    let select_path_button = Paragraph::new("...").block(
        ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .style(
                if app_data.download_data.debug_focus == DebugFocus::SelectPath {
                    Style::default().yellow()
                } else {
                    Style::default()
                },
            ),
    );
    frame.render_widget(select_path_button, layout_2[1]);

    // download button render
    if let Some(mouse_event) = &app_data.mouse_event
        && mouse_event.kind
            == crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left)
        && is_contains_rect(mouse_event.column, mouse_event.row, &layout_2[2]).is_some()
        && app_data.download_data.debug_enter_instant.elapsed().as_millis() > 200
    {
        app_data.download_data.debug_focus = DebugFocus::Download;
        enter_facous = DebugFocus::Download;
        app_data.download_data.debug_enter_instant = Instant::now();
    }
    let download_button = Paragraph::new(download_lable).block(
        ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .style(
                if app_data.download_data.debug_focus == DebugFocus::Download {
                    Style::default().yellow()
                } else {
                    Style::default()
                },
            ),
    );
    frame.render_widget(download_button, layout_2[2]);

    // 处理点击事件
    match enter_facous {
        DebugFocus::SelectPath => {
            rfd::FileDialog::new()
                .set_title(t!("ui.download.debug_tab.path_input"))
                .set_file_name("download.txt")
                .save_file()
                .and_then(|p| {
                    app_data.download_data.debug_input_path =
                        Input::new(p.to_string_lossy().to_string());
                    Some(p.to_string_lossy().to_string().len())
                });
        }
        DebugFocus::Download => {
            if !app_data.download_data.debug_input_url.value().is_empty()
                && !app_data.download_data.debug_input_path.value().is_empty()
            {
                let url = app_data.download_data.debug_input_url.value().to_string();
                let path = app_data.download_data.debug_input_path.value().to_string();
                info!(target: "mc_tui", "added download task: {}", url);
                app_data.download_data.download_pool.add_task(url, path);
            }
        }
        _ => {}
    }
}
