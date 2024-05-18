use super::style::{button_theme, sidebar_button};

use iced::widget::{button, container, row, text, tooltip};
use iced::{Alignment, Element, Font};

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");

    text(codepoint).font(ICON_FONT).into()
}

pub fn action_with_text<'a, Message: Clone + 'a>(
    content: Element<'a, Message>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(
        row![content, text(label)]
            .width(130)
            .align_items(Alignment::Center)
            .spacing(20),
    ).style(sidebar_button);

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(container::rounded_box)
        .into()
    } else {
        action.into()
    }
}

pub fn action<'a, Message: Clone + 'a>(
    content: Element<'a, Message>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(content).style(button_theme);

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(container::rounded_box)
        .into()
    } else {
        action.into()
    }
}

pub fn download_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e800}')
}

pub fn repeat_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e801}')
}

pub fn edit_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e802}')
}

pub fn settings_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e803}')
}

pub fn pause_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e804}')
}

pub fn play_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e805}')
}

pub fn home_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e807}')
}

pub fn forward_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e806}')
}

pub fn backward_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e808}')
}

pub fn delete_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e80a}')
}

pub fn add_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e809}')
}