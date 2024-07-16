use iced::widget::text;
use iced::{Element, Font};

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("icons");

    text(codepoint).font(ICON_FONT).into()
}

pub fn download_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e800}')
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
    icon('\u{0e808}')
}

pub fn forward_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e807}')
}

pub fn backward_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e806}')
}

pub fn add_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e809}')
}

pub fn list_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e80a}')
}

pub fn volume_on<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e80c}')
}

pub fn volume_off<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e80b}')
}

pub fn update_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e80d}')
}

pub fn menu_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0f0c9}')
}

pub fn search_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e80e}')
}

// pub fn repeat_icon<'a, Message>() -> Element<'a, Message> {
//     icon('\u{0e801}')
// }

// pub fn upload_icon<'a, Message>() -> Element<'a, Message> {
//     icon('\u{0e80f}')
// }
