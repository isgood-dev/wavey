use super::style::{button_theme, sidebar_button, transparent_image};

use iced::advanced::image;
use iced::widget::{button, container, image as image_widget, row, text, tooltip, Container};
use iced::{Alignment, Element, Font};

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");

    text(codepoint).font(ICON_FONT).into()
}

pub fn thumbnail_from_bytes<'a, Message>(url: Vec<u8>) -> Container<'a, Message> {
    let handle = image::Handle::from_bytes(url);
    container(image_widget(handle).width(120).height(90)).center_x()
}

pub fn thumbnail_from_path<'a, Message>(url: String) -> Container<'a, Message> {
    // check if path exists
    if !std::path::Path::new(&format!("./assets/thumbnails/{}.jpg", url)).exists() {
        let handle = image::Handle::from_path("./assets/thumbnails/default.jpg");
        return container(image_widget(handle).width(60).height(40))
            .max_width(60)
            .style(transparent_image);
    }

    let handle = image::Handle::from_path(format!("./assets/thumbnails/{}.jpg", url));
    container(image_widget(handle).width(60).height(40))
        .width(60)
        .max_width(60)
        .style(transparent_image)
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
    .style(sidebar_button);

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
