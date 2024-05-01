use iced::{widget::{button, container, row, text, tooltip, Space}, Alignment, Element, Font};

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
        row![container(content).center_x(), text(label)]
            .width(130)
            .align_items(Alignment::Center)
            .spacing(20)
            
    );

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(container::rounded_box)
        .into()
    } else {
        action.style(button::secondary).into()
    }
}

pub fn action<'a, Message: Clone + 'a>(
    content: Element<'a, Message>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(
        container(content)
            .width(30)
            .center_x(),
    );

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(container::rounded_box)
        .into()
    } else {
        action.style(button::secondary).into()
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
