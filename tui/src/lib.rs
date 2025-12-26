pub mod api;
pub mod data;
pub mod log;
pub mod ui;

use std::time::Instant;

use ::log::info;
use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use mc_core::account::base::AccountBase;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, ListItem},
};
use rust_i18n::t;

use crate::data::{AppData, Settings};

rust_i18n::i18n!("locales");

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    info!(target: "MCTui", "MCTui init...");
    let mut app_settings = Settings::load_default().context("load application settings")?;
    let mut app_data = AppData::default();
    app_data.update_from_setting(&app_settings);
    info!(target: "MCTui", "MCTui init OK");
    loop {
        events(&mut app_data, &app_settings)?;
        terminal.draw(|f| render(f, &mut app_data, &mut app_settings))?;
        if app_data.is_quit {
            app_settings
                .save_default()
                .context("save application settings")?;
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, app_data: &mut AppData, app_settings: &mut Settings) {

    let title = if let Some(acc) = &app_settings.account_setting.current_account {
            format!(" MCTui -{}", acc.get_username())
        } else {
            format!(" MCTui -{}",t!("ui.no_account"))
        };

    let outer = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::new().fg(Color::Blue));

    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Min(20),
            Constraint::Length(1),
            Constraint::Percentage(100),
        ])
        .split(outer.inner(frame.area()));

    let separator = Block::default().borders(Borders::LEFT);
    // 程序外框
    frame.render_widget(outer, frame.area());

    // 左侧菜单栏
    let menu_items = vec![
        ListItem::new(t!("ui.menu.game")),
        ListItem::new(t!("ui.menu.download_install")),
        ListItem::new(t!("ui.menu.modpack_manage")),
        ListItem::new(t!("ui.menu.account_manage")),
        ListItem::new(t!("ui.menu.setting")),
    ];
    let menu_items_len = menu_items.len();

    let menu_list = ratatui::widgets::List::new(menu_items)
        .highlight_style(Style::default().bg(Color::LightBlue).fg(Color::Black))
        .highlight_symbol(">> ");

    if let Some(key_event) = &app_data.key_event
        && key_event.is_press()
        && app_data.menu_list_keys_instant.elapsed().as_millis() > 200
    {
        match key_event.code {
            KeyCode::Up => app_data.menu_list_state.select_previous(),
            KeyCode::Down => app_data.menu_list_state.select_next(),
            _ => {}
        }
        app_data.menu_list_keys_instant = Instant::now();
    }
    if let Some(mouse_event) = &app_data.mouse_event
        && mouse_event.kind
            == crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left)
    {
        let x = mouse_event.column;
        let y = mouse_event.row;
        if let Some((_, y)) = api::is_contains_rect(x, y, &layout[0])
            && y < menu_items_len as u16
        {
            app_data.menu_list_state.select(Some(y as usize));
        }
    }

    frame.render_stateful_widget(menu_list, layout[0], &mut app_data.menu_list_state);

    // 分割线
    frame.render_widget(separator, layout[1]);

    // 右侧主区域
    let selected = app_data.menu_list_state.selected().unwrap_or(0);
    match selected {
        0 => ui::ui_game::ui_game_render(frame, layout[2], app_data, app_settings),
        1 => ui::ui_download::ui_download_render(frame, layout[2], app_data),
        2 => ui::ui_modpack_manage::ui_modpack_manage_render(
            frame,
            layout[2],
            app_data,
            app_settings,
        ),
        3 => ui::ui_account_manage::ui_account_manage_render(
            frame,
            layout[2],
            app_data,
            app_settings,
        ),
        4 => ui::ui_setting::ui_setting_render(frame, layout[2], app_data, app_settings),
        _ => {}
    }
}

fn events(app_data: &mut AppData, app_settings: &Settings) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(app_settings.mspt)).context("poll event")? {
        app_data.key_event = None;
        app_data.mouse_event = None;
        app_data.event = None;

        let event = event::read()?;
        app_data.is_read_event = true;
        app_data.event = Some(event.clone());

        match event {
            Event::Key(key_event) => {
                app_data.key_event = Some(key_event);
                app_data.message = format!("{:?}", key_event);
                if key_event.code == KeyCode::Char('q')
                    && key_event.modifiers.contains(KeyModifiers::CONTROL)
                    && !key_event
                        .modifiers
                        .intersects(KeyModifiers::SHIFT | KeyModifiers::ALT)
                {
                    app_data.is_quit = true;
                } else if key_event.code == KeyCode::Esc {
                    app_data.is_quit = true;
                }
            }
            Event::Mouse(mouse_event) => {
                app_data.message = format!("{:?}", mouse_event);
                app_data.mouse_event = Some(mouse_event);
            }
            _ => {}
        }
    } else {
        app_data.is_read_event = false;
    }
    Ok(())
}
