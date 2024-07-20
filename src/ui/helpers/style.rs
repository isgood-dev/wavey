use iced::border::Radius;
use iced::widget::button::{self, Status};
use iced::widget::{container, text};
use iced::{Background, Border, Color, Theme};

pub fn transparent_image(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        ..container::Style::default()
    }
}

pub fn dynamic_colour(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();
    if palette.is_dark {
        container::Style {
            background: Some(Color::from_rgba(0.102, 0.102, 0.102, 1.0).into()),
            ..container::Style::default()
        }
    } else {
        container::Style {
            background: Some(Color::from_rgba(0.8, 0.8, 0.8, 1.0).into()),
            ..container::Style::default()
        }
    }
}

pub fn button_theme(theme: &Theme, _status: Status) -> button::Style {
    let palette = theme.extended_palette();

    if palette.is_dark {
        button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: Color::WHITE,
            ..button::Style::default()
        }
    } else {
        button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: Color::BLACK,
            ..button::Style::default()
        }
    }
}

pub fn sidebar_button(theme: &Theme, _status: Status) -> button::Style {
    let palette = theme.extended_palette();

    if palette.is_dark {
        button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: Color::from_rgb8(192, 192, 192),
            ..button::Style::default()
        }
    } else {
        button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: Color::from_rgb8(64, 64, 64),
            ..button::Style::default()
        }
    }
}

pub fn sidebar_text(theme: &Theme) -> text::Style {
    let palette = theme.extended_palette();

    if palette.is_dark {
        text::Style {
            color: Some(Color::from_rgb8(192, 192, 192)),
            ..text::Style::default()
        }
    } else {
        text::Style {
            color: Some(Color::from_rgb8(64, 64, 64)),
            ..text::Style::default()
        }
    }
}

pub fn track_list_item(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    let offset = offset_colour(palette.background.base.color.into(), false, palette.is_dark);

    container::Style {
        background: Some(iced::Background::Color(offset)),
        border: Border {
            radius: Radius::new(8.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn offset_colour(colour: Color, hovered: bool, is_dark: bool) -> Color {
    let r = colour.r;
    let g = colour.g;
    let b = colour.b;
    let a = colour.a;

    if hovered {
        if is_dark {
            Color::from_rgba(r - 0.05, g - 0.05, b - 0.05, a)
        } else {
            Color::from_rgba(r + 0.05, g + 0.05, b + 0.05, a)
        }
    } else {
        if is_dark {
            Color::from_rgba(r + 0.03, g + 0.03, b + 0.03, a)
        } else {
            Color::from_rgba(r - 0.03, g - 0.03, b - 0.03, a)
        }
    }
}
