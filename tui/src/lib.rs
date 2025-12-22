pub mod data;

use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame, layout::{Constraint, Layout}, style::{Color, Style}, widgets::{Block, Borders, ListItem}
};

use crate::data::{AppData, Settings};

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app_data = AppData::default();
    let app_settings = Settings::load_default().context("load application settings")?;
    loop {
        events(&mut app_data, &app_settings)?;
        terminal.draw(|f| render(f, &mut app_data))?;
        if app_data.is_quit {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, app_data: &mut AppData) {
    
    let outer = Block::default()
        .borders(Borders::ALL)
        .title(" MCTui ")
        .border_style(Style::new().fg(Color::Blue));
    
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(1), Constraint::Percentage(100)])
        .split(outer.inner(frame.area()));

    let separator = Block::default().borders(Borders::LEFT);
    // 程序外框
    frame.render_widget(outer, frame.area());

    // 左侧菜单栏
    let menu_items = vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
    ];

    let menu_list = ratatui::widgets::List::new(menu_items)
        .highlight_style(Style::default().bg(Color::LightBlue).fg(Color::Black))
        .highlight_symbol(">> ");
    frame.render_stateful_widget(menu_list, layout[0], &mut app_data.menu_list_state);

    // 分割线
    frame.render_widget(separator, layout[1]);

    // 右侧主区域
    frame.render_widget(app_data.message.as_str(), layout[2]);
}

fn events(app_data: &mut AppData, app_settings: &Settings) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(app_settings.mspt)).context("poll event")? {
        match event::read()? {
            Event::Key(key_event) if key_event.is_press() => {
                match key_event.code {
                    KeyCode::Char('q') => app_data.is_quit = true,
                    KeyCode::Up => app_data.menu_list_state.select_previous(),
                    KeyCode::Down => app_data.menu_list_state.select_next(),
                    _ => {}
                }
            }
            Event::Mouse(event) => {
                app_data.message = format!("{:?}", event);
            }
            _ => {}
        }
    }
    Ok(())
}
