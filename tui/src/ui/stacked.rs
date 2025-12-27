use mc_core::account::base::AccountBase;
use rat_theme4::StyleName;
use rat_theme4::palette::Colors;
use rat_widget::statusline_stacked::{SLANT_BL_TR, StatusLineStacked};
use ratatui_core::text::Span;
use ratatui_core::{buffer::Buffer, layout::Rect, style::Style};
use ratatui_core::widgets::Widget;
use rust_i18n::t;

use crate::data::{AppData, Settings};

pub fn stacked_render(
    area: Rect,
    buf: &mut Buffer,
    app_data: &mut AppData,
    app_settings: &mut Settings,
) {
    let pal = &app_settings.theme.p;

    // let color_0 = pal.color(Colors::Gray, 3);
    // let color_1 = pal.color(Colors::Cyan, 3);
    let color_3 = pal.color(Colors::Cyan, 0);
    let color_4 = pal.color(Colors::Cyan, 7);

    let (account_name, account_type) = get_current_account_name_and_type(app_settings);

    StatusLineStacked::new()
        .style(app_settings.theme.style(Style::STATUS_BASE))
        .end(
            Span::from(account_name)
                .style(Style::new().fg(pal.color(Colors::TextDark, 3)).bg(color_3)),
            Span::from(SLANT_BL_TR).style(Style::new().fg(color_3).bg(color_4)),
        )
        .end(
            "",
            Span::from(SLANT_BL_TR).style(Style::new().fg(color_4).bg(color_3)),
        )
        .end(
            Span::from(account_type)
                .style(Style::new().fg(pal.color(Colors::TextDark, 3)).bg(color_3)),
            Span::from(SLANT_BL_TR).style(Style::new().fg(color_3).bg(color_4)),
        )
        .end("", Span::from(SLANT_BL_TR).style(Style::new().fg(color_4)))
        .render(area, buf);

}

fn get_current_account_name_and_type(app_settings: &Settings) -> (String, String) {
    if let Some(account) = &app_settings.account_setting.current_account {
        (account.get_username().to_string(), t!(format!("ui.stacked.{}", account.get_type())).to_string())
    } else {
        (String::new(), t!("ui.stacked.no_account").to_string())
    }
}