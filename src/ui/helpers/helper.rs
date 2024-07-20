use crate::core::format;

use super::{icons, style};

use iced::advanced::image;
use iced::widget::{
    button, center, container, horizontal_space, image as image_widget, mouse_area, opaque, row,
    stack, text, tooltip, Container, Space,
};
use iced::{Alignment, Color, Element};

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
            .align_y(Alignment::Center)
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

pub fn track_list_item<'a, Message: Clone + 'a>(
    thumbnail_handle: iced::advanced::image::Handle,
    label: &'a str,
    duration: &'a str,
    play_event: Message,
    edit_event: Message,
    add_playlist_event: Message,
    hovered: bool,
) -> Element<'a, Message> {
    let mut content = row![]
        .align_y(Alignment::Center)
        .push(
            button(icons::play_icon())
                .on_press(play_event)
                .style(style::button_theme),
        )
        .push(Space::with_width(5))
        .push(thumbnail(thumbnail_handle))
        .push(Space::with_width(10))
        .push(text(label))
        .push(horizontal_space())
        .push(text(format::duration(duration.parse().unwrap())))
        .push(Space::with_width(15));

    if hovered {
        content = content.push(
            button(icons::edit_icon())
                .on_press(edit_event)
                .style(style::button_theme),
        );

        content = content.push(
            button(icons::add_icon())
                .on_press(add_playlist_event)
                .style(style::button_theme),
        );
    }

    container(content).style(style::track_list_item).into()
}
