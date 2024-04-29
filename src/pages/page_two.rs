use iced::{widget, Command};

pub struct State {
    counter: i32,
}

#[derive(Debug, Clone)]
pub enum Event {
    Increment,
}

impl State {
    pub fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::Increment => {
                self.counter += 1;

                Command::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        widget::Button::new("Click 2")
            .on_press(Event::Increment)
            .into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self { counter: 0 }
    }
}