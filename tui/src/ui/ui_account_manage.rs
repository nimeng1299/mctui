use std::{time::Instant, vec};

use crossterm::event::{KeyCode, KeyModifiers};
use log::{info, warn};
use mc_core::account::{Account, base::AccountBase, offline_account::OfflineAccount};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs},
};
use rust_i18n::t;
use tui_input::{Input, backend::crossterm::EventHandler};

use crate::{
    api::is_contains_rect,
    data::{
        AppData, Settings,
        account_data::{AccountShowMode, AddOfflineAccountFocus},
    },
};

pub fn ui_account_manage_render(
    frame: &mut Frame,
    area: Rect,
    app_data: &mut AppData,
    setting: &mut Settings,
) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Percentage(100)])
        .split(area);

    let mut tabs_item = vec![
        t!("ui.account_manage.account_table"),
        t!("ui.account_manage.online_login"),
        t!("ui.account_manage.offline_login"),
        t!("ui.account_manage.other_login"),
    ];
    if app_data.account_data.table_state.selected().is_some()
        && !setting.account_setting.all_accounts.is_empty()
    {
        tabs_item.push(t!("ui.account_manage.delete_select"));
        tabs_item.push(t!("ui.account_manage.update_select"));
    }

    let tabs_item_str_len = tabs_item
        .iter()
        .map(|f| Line::from(f.clone()).width())
        .collect::<Vec<_>>();

    if let Some(key_event) = &app_data.key_event
        && key_event.is_press()
        && !app_data.account_data.is_input_mode
        && app_data
            .account_data
            .tabs_keys_instant
            .elapsed()
            .as_millis()
            > 200
    {
        match key_event.code {
            KeyCode::Left => {
                if app_data.account_data.seleted_account_index > 0 {
                    app_data.account_data.seleted_account_index -= 1;
                }
                app_data.account_data.tabs_keys_instant = Instant::now();
            }
            KeyCode::Right => {
                if app_data.account_data.seleted_account_index < tabs_item.len() - 1 {
                    app_data.account_data.seleted_account_index += 1;
                }
                app_data.account_data.tabs_keys_instant = Instant::now();
            }
            KeyCode::Enter => {}
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
                        app_data.account_data.seleted_account_index = i;
                        break;
                    }
                    current_len += *len as u16 + 3; // 3 is padding and divider
                }
            }
        }
    }

    match app_data.account_data.seleted_account_index {
        0 => {
            app_data.account_data.show_mode = AccountShowMode::Table;
        }
        1 => {
            app_data.account_data.show_mode = AccountShowMode::AddOnlineAccount;
        }
        2 => {
            app_data.account_data.show_mode = AccountShowMode::AddOfflineAccount;
        }
        3 => {
            app_data.account_data.show_mode = AccountShowMode::AddOtherAccount;
        }
        _ => {}
    }

    // render tabs
    let tab = Tabs::new(tabs_item)
        .style(Style::default().white())
        .highlight_style(Style::default().yellow())
        .select(app_data.account_data.seleted_account_index);

    frame.render_widget(tab, layout[0]);

    match app_data.account_data.show_mode {
        AccountShowMode::Table => {
            render_table(frame, layout[1], app_data, setting);
        }
        AccountShowMode::AddOnlineAccount => {}
        AccountShowMode::AddOfflineAccount => {
            render_add_offline_account(frame, layout[1], app_data, setting);
        }
        AccountShowMode::AddOtherAccount => {}
    };
}

fn render_table(frame: &mut Frame, area: Rect, app_data: &mut AppData, setting: &mut Settings) {
    if let Some(key_event) = &app_data.key_event
        && key_event.is_press()
        && !app_data.account_data.is_input_mode
        && app_data
            .account_data
            .tabs_keys_instant
            .elapsed()
            .as_millis()
            > 200
    {
        match key_event.code {
            KeyCode::Char('w') => {
                app_data.account_data.table_state.select_previous();
                app_data.account_data.tabs_keys_instant = Instant::now();
            }
            KeyCode::Char('s') => {
                app_data.account_data.table_state.select_next();
                app_data.account_data.tabs_keys_instant = Instant::now();
            }
            KeyCode::Enter => {
                if let Some(index) = app_data.account_data.table_state.selected()
                    && index < setting.account_setting.all_accounts.len()
                    {
                    setting.account_setting.current_account = Some(setting.account_setting.all_accounts[index].clone());
                }
            }
            _ => {}
        }
    }

    let header = Row::new(vec![
        Cell::from(t!("ui.account_manage.account_name")),
        Cell::from(t!("ui.account_manage.account_type")),
        Cell::from(t!("ui.account_manage.current_account")),
    ])
    .style(Style::default().fg(Color::Yellow))
    .bottom_margin(1);

    let mut rows = vec![];
    for acc in &setting.account_setting.all_accounts {
        rows.push(Row::new(vec![
            Cell::from(acc.get_username()),
            Cell::from(t!(format!("ui.account_manage.{}", acc.get_type()))),
            Cell::from(
                if acc == setting.account_setting.current_account.as_ref().unwrap() {
                    "●"
                } else {
                    ""
                },
            ),
        ]));
    }

    let table_widtes = vec![
        Constraint::Percentage(50),
        Constraint::Percentage(30),
        Constraint::Percentage(20),
    ];
    let table = Table::new(rows, table_widtes).header(header).block(
        Block::default()
            .title(t!("ui.account_manage.account_manage"))
            .borders(Borders::ALL),
    ).row_highlight_style(Style::default().fg(Color::LightMagenta));
    frame.render_stateful_widget(table, area, &mut app_data.account_data.table_state);
}

fn render_add_online_account(
    frame: &mut Frame,
    area: Rect,
    app_data: &mut AppData,
    setting: &mut Settings,
) {
    let block = Block::default()
        .title(t!("ui.account_manage.add_online_account"))
        .borders(Borders::ALL);

    frame.render_widget(block, area);
}

fn render_add_offline_account(
    frame: &mut Frame,
    area: Rect,
    app_data: &mut AppData,
    setting: &mut Settings,
) {
    let mut enter_facous = AddOfflineAccountFocus::None;

    if matches!(
        app_data.account_data.add_offline_account_focus,
        AddOfflineAccountFocus::UserNameInput
    ) {
        app_data.account_data.is_input_mode = true;
    } else {
        app_data.account_data.is_input_mode = false;
    }

    let block = Block::default()
        .title(t!("ui.account_manage.add_offline_account"))
        .borders(Borders::ALL);

    let inarea = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(3)])
        .split(inarea);

    if let Some(key_event) = &app_data.key_event
        && key_event.is_press()
        && app_data.keys_instant.elapsed().as_millis() > 200
    {
        app_data.keys_instant = Instant::now();
        match key_event.code {
            KeyCode::Tab => {
                app_data.account_data.add_offline_account_focus =
                    app_data.account_data.add_offline_account_focus.tab();
            }
            KeyCode::Enter
                if matches!(
                    app_data.account_data.add_offline_account_focus,
                    AddOfflineAccountFocus::Add
                ) =>
            {
                enter_facous = app_data.account_data.add_offline_account_focus.clone();
            }
            _ => {}
        }
    }

    // username input
    if let Some(mouse_event) = &app_data.mouse_event
        && mouse_event.kind
            == crossterm::event::MouseEventKind::Down(crossterm::event::MouseButton::Left)
                && app_data.mouse_instant.elapsed().as_millis() > 200
    {
        app_data.mouse_instant = Instant::now();
        if is_contains_rect(mouse_event.column, mouse_event.row, &layout[0]).is_some() {
            app_data.account_data.add_offline_account_focus = AddOfflineAccountFocus::UserNameInput;
        } else if is_contains_rect(mouse_event.column, mouse_event.row, &layout[1]).is_some() {
            app_data.account_data.add_offline_account_focus = AddOfflineAccountFocus::Add;
            enter_facous = AddOfflineAccountFocus::Add;
        }
    }
    if let Some(event) = &app_data.event
        && app_data.is_read_event
    {
        if app_data.account_data.add_offline_account_focus == AddOfflineAccountFocus::UserNameInput
        {
            app_data
                .account_data
                .add_offline_username_input
                .handle_event(event);
        }
    }

    let username_input = Paragraph::new(app_data.account_data.add_offline_username_input.value())
        .style(Style::default())
        .block(
            ratatui::widgets::Block::default()
                .title(t!("ui.account_manage.username_input"))
                .borders(ratatui::widgets::Borders::ALL)
                .style(
                    if app_data.account_data.add_offline_account_focus
                        == AddOfflineAccountFocus::UserNameInput
                    {
                        Style::default().yellow()
                    } else {
                        Style::default()
                    },
                ),
        );
    frame.render_widget(username_input, layout[0]);
    if app_data.account_data.add_offline_account_focus == AddOfflineAccountFocus::UserNameInput {
        let width = layout[0].width.max(3) - 3;
        let scroll = app_data
            .account_data
            .add_offline_username_input
            .visual_scroll(width as usize);
        let x = app_data
            .account_data
            .add_offline_username_input
            .visual_cursor()
            .max(scroll)
            - scroll
            + 1;
        frame.set_cursor_position((layout[0].x + x as u16, layout[0].y + 1))
    }

    let add = Paragraph::new(t!("ui.account_manage.add"))
        .style(Style::default())
        .alignment(Alignment::Center)
        .block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .style(
                    if app_data.account_data.add_offline_account_focus
                        == AddOfflineAccountFocus::Add
                    {
                        Style::default().yellow()
                    } else {
                        Style::default()
                    },
                ),
        );
    frame.render_widget(add, layout[1]);

    match enter_facous {
        AddOfflineAccountFocus::Add => {
            let username = app_data
                .account_data
                .add_offline_username_input
                .value()
                .to_string();
            if !username.is_empty() {
                let offline_account = OfflineAccount::new(username.clone());
                setting
                    .account_setting
                    .all_accounts
                    .push(Account::Offline(offline_account.clone()));
                setting.account_setting.current_account = Some(Account::Offline(offline_account));
                let _ = setting.save_default();
                info!(target:"MCTui", "Added offline account: {}.", username);
                app_data.account_data.add_offline_username_input = Input::default();
                app_data.account_data.show_mode = AccountShowMode::Table;
            }else {
                warn!(target:"MCTui", "Add offline account failed: Username is empty.");
            }
        }
        _ => {}
    }
}
