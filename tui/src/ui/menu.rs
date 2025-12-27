use std::borrow::Cow;

use rat_theme4::WidgetStyle;
use rat_menu::menuline::MenuLine;
use rat_widget::menu::MenuLineState;
use ratatui_core::{buffer::Buffer, layout::Rect};
use ratatui_core::widgets::StatefulWidget;
use rust_i18n::t;

use crate::data::{AppData, Settings};

pub fn menu_render(
    area: Rect,
    buf: &mut Buffer,
    app_data: &mut AppData,
    app_settings: &mut Settings,
) {
    MenuLine::new()
        .item_parsed(menu_item(t!("ui.menu.game"), 0, &app_data.menu_selected).as_str())
        .item_parsed(menu_item(t!("ui.menu.download_install"), 1, &app_data.menu_selected).as_str())
        .item_parsed(menu_item(t!("ui.menu.quit"), 2, &app_data.menu_selected).as_str())
        .styles(app_settings.theme.style(WidgetStyle::MENU))
        .focus_style(app_settings.theme.p.primary(3))
        .render(area, buf, &mut app_data.menu_selected);
}

fn menu_item(item: Cow<'static, str>, index: usize, stage: &MenuLineState) -> String {
    if stage.selected() == Some(index) {
        format!("> {}", item)
    } else {
        format!("  {}", item)
    }
}
