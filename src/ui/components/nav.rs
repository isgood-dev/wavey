use super::helpers;
use super::style;

use iced::{
    widget::{container, horizontal_space, row, Space},
    Task,
};

pub struct State {}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    CollapseSidebar,
    CheckUpdates,
    UpdaterStarted(()),
}

impl State {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: Event) -> Task<Event> {
        match message {
            Event::CollapseSidebar => Task::none(),
            Event::CheckUpdates => Task::perform(start_updater(), Event::UpdaterStarted),
            Event::UpdaterStarted(_) => Task::none(),
        }
    }

    pub fn view(&self) -> iced::Element<Event> {
        let content = container(row![
            helpers::action(
                helpers::menu_icon(),
                "Collapse",
                Some(Event::CollapseSidebar)
            ),
            horizontal_space(),
            helpers::action(
                helpers::update_icon(),
                "Check for updates",
                Some(Event::CheckUpdates)
            ),
            Space::with_width(10),
        ])
        .style(style::dynamic_colour);

        content.into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

async fn start_updater() {
    // either get executable from release folder or in root
    let release_path = "./target/release/updater";
    let root_path = "./updater";

    let executable = if std::path::Path::new(release_path).exists() {
        release_path
    } else if std::path::Path::new(root_path).exists() {
        root_path
    } else {
        log::error!("Updater not found");
        return;
    };

    let output = tokio::process::Command::new(executable).output().await;

    match output {
        Ok(output) => {
            if output.status.success() {
                log::info!("Update successful");
            } else {
                log::error!("Update failed");
            }
        }
        Err(_) => {
            log::error!("Update failed");
        }
    }
}
