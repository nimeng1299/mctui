pub mod api;
pub mod data;
pub mod event;
pub mod log;
pub mod ui;

use ::log::{error, info};
use anyhow::{Context, Result};
use rat_event::{crossterm::modifiers::CONTROL, ct_event, try_flow};
use rat_menu::{event::MenuOutcome, menuline};
use rat_salsa::{
    Control, RunConfig,
    poll::{PollCrossterm, PollRendered},
    run_tui,
};
use ratatui_core::{buffer::Buffer, layout::{Constraint, Layout, Rect}};
use rust_i18n::t;

use crate::{
    data::{AppData, Settings},
    event::AppEvent,
};

rust_i18n::i18n!("locales");

pub fn run() -> Result<()> {
    info!(target: "MCTui", "MCTui init...");
    let mut app_settings = Settings::load_default().context("load application settings")?;
    let mut app_data = AppData::default();
    info!(target: "MCTui", "MCTui init OK");

    run_tui(
        init,
        render,
        events,
        errors,
        &mut app_settings,
        &mut app_data,
        RunConfig::default()?.poll(PollCrossterm).poll(PollRendered),
    )?;

    Ok(())
}

fn init(_app_data: &mut AppData, _app_settings: &mut Settings) -> Result<()> {
    Ok(())
}

fn render(
    area: Rect,
    buf: &mut Buffer,
    app_data: &mut AppData,
    app_settings: &mut Settings,
) -> Result<()> {

    let l1 = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ]).split(area);

    ui::menu::menu_render(l1[0], buf, app_data, app_settings);
    
    
    match app_data.menu_selected.selected() {
        Some(1) => {
            ui::download::download_render(l1[1], buf, app_data, app_settings);
        }
        _ => {}
    }

    ui::stacked::stacked_render(l1[2], buf, app_data, app_settings);

    Ok(())
}

fn events(
    event: &AppEvent,
    app_data: &mut AppData,
    app_settings: &mut Settings,
) -> Result<Control<AppEvent>> {
    let r= match event {
        AppEvent::Event(event) => {
            // 菜单处理
            try_flow!(
                match menuline::handle_events(&mut app_data.menu_selected, true, event){
                    MenuOutcome::Selected(v) => {
                        app_data.menu_selected.select(Some(v));
                        Control::Changed
                    }
                    MenuOutcome::Activated(2) => Control::Quit,
                _ => Control::Continue
                }
            );

            match app_data.menu_selected.selected() {
                Some(0) => {},
                Some(1) => {
                    // Download 菜单处理
                    try_flow!(
                        match menuline::handle_events(&mut app_data.download_data.download_selected, true, event){
                            MenuOutcome::Selected(v) => {
                                app_data.download_data.download_selected.select(Some(v));
                                Control::Changed
                            }
                        _ => Control::Continue
                        }
                    );
                },
                _ => {}
            }

            match event {
                ct_event!(key press CONTROL-'q') => {Control::Quit},
                ct_event!(keycode press Esc) => {Control::Quit},
                
                _ => {Control::Continue}
            }


        },
        _ => {Control::Continue}
    };

    Ok(r)
}

fn errors(
    err: anyhow::Error,
    _app_data: &mut AppData,
    _app_settings: &mut Settings,
) -> Result<Control<AppEvent>> {
    error!(target: "MCTui", "Error occurred: {:?}", err);
    Ok(Control::Changed)
}
