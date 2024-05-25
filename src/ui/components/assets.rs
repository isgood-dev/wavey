use super::style;

use iced::advanced::image;
use iced::widget::{
    button, center, container, image as image_widget, mouse_area, opaque, row, stack, text,
    tooltip, Container,
};
use iced::{Alignment, Color, Element, Font};

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");

    text(codepoint).font(ICON_FONT).into()
}

pub fn thumbnail_from_bytes<'a, Message>(url: Vec<u8>) -> Container<'a, Message> {
    let handle = image::Handle::from_bytes(url);
    container(image_widget(handle).width(120).height(90)).center_x()
}

pub fn thumbnail<'a, Message>(handle: image::Handle) -> Container<'a, Message> {
    // check if path exists
    container(image_widget(handle).width(60).height(40))
        .max_width(60)
        .style(style::transparent_image)
}

pub fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        mouse_area(center(opaque(content)).style(|_theme| {
            container::Style {
                background: Some(
                    Color {
                        a: 0.8,
                        ..Color::BLACK
                    }
                    .into(),
                ),
                ..container::Style::default()
            }
        }))
        .on_press(on_blur)
    ]
    .into()
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
    )
    .style(style::sidebar_button);

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
    let action = button(content).style(style::button_theme);

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
