use iced::Command;

mod homepage;
mod page_two;

pub struct Pages {
    pub current_page: Page,

    homepage: homepage::State,
    page_two: page_two::State,
}

#[derive(Default)]
pub enum Page {
    #[default]
    Homepage,
    PageTwo,
}
pub enum Event {
    HomePagePressed(homepage::Event),
    PageTwoPressed(page_two::Event),
}

impl Pages {
    fn update(&mut self, message: Event) -> Command<Event> {
        match message {
            Event::HomePagePressed(x) => self.homepage.update(x).map(Event::HomePagePressed),
            Event::PageTwoPressed(x) => self.page_two.update(x).map(Event::PageTwoPressed),
        }
    }

    fn view(&self) -> iced::Element<Event> {
        match &self.current_page {
            Page::Homepage => {
                self.homepage.view().map(Event::HomePagePressed)
            }
            Page::PageTwo => {
                self.page_two.view().map(Event::PageTwoPressed)
            }
        }
    }
}

impl Default for Pages {
    fn default() -> Self {
        Self {
            current_page: Default::default(),
            homepage: Default::default(),
            page_two: Default::default(),
        }
    }
}