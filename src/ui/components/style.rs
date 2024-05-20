use iced::widget::button::{self, Status};
use iced::widget::{container, text};
use iced::{Background, Color, Theme};

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

pub fn sidebar_text(_theme: &Theme) -> text::Style {
    text::Style {
        color: Some(Color::from_rgb8(192, 192, 192)),
        ..text::Style::default()
    }
}