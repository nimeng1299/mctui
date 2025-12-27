use std::borrow::Cow;

use mc_core::{install::minecraft::get_all_minecraft_versions, statue::Status};
use rat_theme4::{StyleName, WidgetStyle};
use rat_widget::{menu::{MenuLine, MenuLineState}, scrolled::{Scroll, ScrollbarPolicy}, textarea::TextArea};
use ratatui_core::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::Style};
use ratatui_core::widgets::StatefulWidget;
use ratatui_widgets::block::Block;
use rust_i18n::t;

use crate::data::{AppData, Settings};

pub fn download_render(
    area: Rect,
    buf: &mut Buffer,
    app_data: &mut AppData,
    app_settings: &mut Settings,
) {
    let l1 = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
    ]).split(area);

    MenuLine::new()
        .item_parsed(menu_item(t!("ui.download.minecraft"), 0, &app_data.download_data.download_selected).as_str())
        .item_parsed(menu_item(t!("ui.download.modpack"), 1, &app_data.download_data.download_selected).as_str())
        .item_parsed(menu_item(t!("ui.download.mod"), 2, &app_data.download_data.download_selected).as_str())
        .styles(app_settings.theme.style(WidgetStyle::MENU))
        .focus_style(app_settings.theme.p.primary(2))
        .render(l1[0], buf, &mut app_data.download_data.download_selected);  

    match app_data.download_data.download_selected.selected() {
        Some(0) => {
            render_minecraft_download_area(l1[1], buf, app_data, app_settings);
        }
        // Some(1) => {
        //     render_modpack_download_area(l1[1], buf, app_data, app_settings);
        // }
        // Some(2) => {
        //     render_mod_download_area(l1[1], buf, app_data, app_settings);
        // }
        _ => {}
    }
}

fn menu_item(item: Cow<'static, str>, index: usize, stage: &MenuLineState) -> String {
    if stage.selected() == Some(index) {
        format!("> {}", item)
    } else {
        format!("  {}", item)
    }
}

fn render_minecraft_download_area(
    area: Rect,
    buf: &mut Buffer,
    app_data: &mut AppData,
    app_settings: &mut Settings,
) {
    match get_all_minecraft_versions(&mut app_data.download_data.minecraft_downloader){
        Status::Success(data) =>{
            app_data.download_data.text_state.set_text(format!("{:#?}", data));
            TextArea::new()
            .style(app_settings.theme.style(WidgetStyle::TEXTVIEW))
            .vscroll(Scroll::new().policy(ScrollbarPolicy::Collapse))
            .block(Block::bordered()
                .title(t!("ui.download.minecraft_versions"))
                .border_style(app_settings.theme.style_style(Style::CONTAINER_BORDER_FG))
                .title_style(app_settings.theme.style_style(Style::CONTAINER_BORDER_FG)),
            ).text_wrap(rat_widget::textarea::TextWrap::Word(0))
            .render(area, buf, &mut app_data.download_data.text_state);
        }
        Status::Progress(_) =>
        {
            app_data.download_data.text_state.set_text(t!("ui.download.downloading"));  
            TextArea::new()
            .style(app_settings.theme.style(WidgetStyle::TEXTVIEW))
            .vscroll(Scroll::new().policy(ScrollbarPolicy::Collapse))
            .block(Block::bordered()
                .title(t!("ui.download.minecraft_versions"))
                .border_style(app_settings.theme.style_style(Style::CONTAINER_BORDER_FG))
                .title_style(app_settings.theme.style_style(Style::CONTAINER_BORDER_FG)),
            ).text_wrap(rat_widget::textarea::TextWrap::Word(0))
            .render(area, buf, &mut app_data.download_data.text_state);
        }
        Status::Failed(e) => {
            app_data.download_data.text_state.set_text(e);
            TextArea::new()
            .style(app_settings.theme.style(WidgetStyle::TEXTVIEW))
            .vscroll(Scroll::new().policy(ScrollbarPolicy::Collapse))
            .block(Block::bordered()
                .title(t!("ui.download.minecraft_versions"))
                .border_style(app_settings.theme.style_style(Style::CONTAINER_BORDER_FG))
                .title_style(app_settings.theme.style_style(Style::CONTAINER_BORDER_FG)),
            ).text_wrap(rat_widget::textarea::TextWrap::Word(0))
            .render(area, buf, &mut app_data.download_data.text_state);
        }
    }
}