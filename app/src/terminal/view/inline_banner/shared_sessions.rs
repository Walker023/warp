//! The rendering logic for shared session banners.
use chrono::{DateTime, Datelike, Local};
use warpui::elements::{
    Border, ConstrainedBox, Container, CornerRadius, CrossAxisAlignment, Flex, MainAxisSize,
    ParentElement, Radius, Rect, Shrinkable, Text,
};
use warpui::fonts::{Properties, Weight};
use warpui::Element;

use crate::appearance::Appearance;
use crate::i18n::t;

fn render_inline_shared_session_banner(
    is_active: bool,
    label: String,
    datetime: DateTime<Local>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let border_fill = if is_active {
        appearance.theme().terminal_colors().normal.red.into()
    } else {
        appearance.theme().surface_2()
    };

    let left_line = ConstrainedBox::new(Rect::new().with_background(border_fill).finish())
        .with_height(1.)
        .finish();

    let right_line = ConstrainedBox::new(Rect::new().with_background(border_fill).finish())
        .with_height(1.)
        .finish();

    let today = Local::now();
    let is_today = datetime.year() == today.year() && datetime.ordinal() == today.ordinal();
    let day_str = if is_today {
        t!("terminal_ui.inline_banner.shared_session.today").to_string()
    } else {
        let month = match datetime.month() {
            1 => t!("terminal_ui.inline_banner.shared_session.months.january"),
            2 => t!("terminal_ui.inline_banner.shared_session.months.february"),
            3 => t!("terminal_ui.inline_banner.shared_session.months.march"),
            4 => t!("terminal_ui.inline_banner.shared_session.months.april"),
            5 => t!("terminal_ui.inline_banner.shared_session.months.may"),
            6 => t!("terminal_ui.inline_banner.shared_session.months.june"),
            7 => t!("terminal_ui.inline_banner.shared_session.months.july"),
            8 => t!("terminal_ui.inline_banner.shared_session.months.august"),
            9 => t!("terminal_ui.inline_banner.shared_session.months.september"),
            10 => t!("terminal_ui.inline_banner.shared_session.months.october"),
            11 => t!("terminal_ui.inline_banner.shared_session.months.november"),
            12 => t!("terminal_ui.inline_banner.shared_session.months.december"),
            _ => unreachable!("chrono returned an invalid month"),
        };
        t!(
            "terminal_ui.inline_banner.shared_session.date",
            month = month,
            day = datetime.day()
        )
        .to_string()
    };

    let time_str = datetime.format("%H:%M").to_string();
    let datetime_str = t!(
        "terminal_ui.inline_banner.shared_session.datetime",
        date = day_str,
        time = time_str
    )
    .to_string();

    let pill = Container::new(
        Flex::row()
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_child(
                Container::new(
                    Text::new_inline(
                        label,
                        appearance.ui_font_family(),
                        appearance.monospace_font_size(),
                    )
                    .with_color(appearance.theme().active_ui_text_color().into())
                    .with_style(Properties::default().weight(Weight::Bold))
                    .finish(),
                )
                .with_padding_right(8.)
                .finish(),
            )
            .with_child(
                Text::new_inline(
                    datetime_str,
                    appearance.ui_font_family(),
                    appearance.monospace_font_size(),
                )
                .with_color(
                    appearance
                        .theme()
                        .sub_text_color(appearance.theme().surface_2())
                        .into(),
                )
                .finish(),
            )
            .finish(),
    )
    .with_border(Border::all(1.).with_border_fill(border_fill))
    .with_corner_radius(CornerRadius::with_all(Radius::Percentage(50.)))
    .with_horizontal_padding(12.)
    .with_vertical_margin(4.)
    .finish();

    Flex::row()
        .with_cross_axis_alignment(CrossAxisAlignment::Center)
        .with_main_axis_size(MainAxisSize::Max)
        .with_child(Shrinkable::new(1., left_line).finish())
        .with_child(pill)
        .with_child(Shrinkable::new(1., right_line).finish())
        .finish()
}

pub fn render_inline_shared_session_started_banner(
    is_active: bool,
    is_shared_ambient_agent_session: bool,
    is_remote_control: bool,
    started_at: DateTime<Local>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let label = if is_shared_ambient_agent_session {
        t!("terminal_ui.inline_banner.shared_session.environment_started").to_string()
    } else if is_remote_control {
        t!("terminal_ui.inline_banner.shared_session.remote_control_active").to_string()
    } else {
        t!("terminal_ui.inline_banner.shared_session.sharing_started").to_string()
    };
    render_inline_shared_session_banner(is_active, label, started_at, appearance)
}

pub fn render_inline_shared_session_ended_banner(
    is_shared_ambient_agent_session: bool,
    is_remote_control: bool,
    ended_at: DateTime<Local>,
    appearance: &Appearance,
) -> Box<dyn Element> {
    let label = if is_shared_ambient_agent_session {
        t!("terminal_ui.inline_banner.shared_session.environment_ended").to_string()
    } else if is_remote_control {
        t!("terminal_ui.inline_banner.shared_session.remote_control_stopped").to_string()
    } else {
        t!("terminal_ui.inline_banner.shared_session.sharing_ended").to_string()
    };
    render_inline_shared_session_banner(false, label, ended_at, appearance)
}
