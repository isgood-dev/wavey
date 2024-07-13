use iced::{
    widget::{button, column, container, text},
    Alignment, Length, Task,
};

use tokio::fs;

fn main() -> iced::Result {
    iced::application("wavey updater", Updater::update, Updater::view)
        .window_size((500.0, 250.0))
        .run()
}

struct Updater {
    complete: bool,
    versions_match: bool,
    error: bool,
    error_type: Option<UpdateError>,
}

#[derive(Debug, Clone)]
enum Message {
    BeginPressed,
    Finished(Result<UpdateState, UpdateError>),
}

impl Updater {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BeginPressed => Task::perform(start_update(), Message::Finished),
            Message::Finished(status) => {
                match status {
                    Ok(UpdateState::VersionMatch) => {
                        self.versions_match = true;
                    }
                    Ok(UpdateState::Completed) => {
                        self.complete = true;
                    }
                    Err(error) => {
                        self.error = true;

                        match error {
                            UpdateError::ReqwestError => {
                                self.error_type = Some(UpdateError::ReqwestError);
                            }
                            UpdateError::IoError => {
                                self.error_type = Some(UpdateError::IoError);
                            }
                            UpdateError::WriteBytesFailed => {
                                self.error_type = Some(UpdateError::WriteBytesFailed);
                            }
                            UpdateError::ExtractFailed => {
                                self.error_type = Some(UpdateError::ExtractFailed);
                            }
                        }
                    }
                }

                Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        if self.versions_match {
            return container(
                column![
                    text("No updates available").size(24),
                    text("Your version of wavey is the latest."),
                ]
                .align_x(Alignment::Center)
                .spacing(20),
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
        }

        if self.error {
            return container(
                column![
                    text("Update failed").size(24),
                    text("An error occurred while updating the app."),
                    text(match self.error_type {
                        Some(UpdateError::ReqwestError) => "An error occured trying to retriveing the update package. Are you connected to the internet?",
                        Some(UpdateError::IoError) => "An IO error occurred.",
                        Some(UpdateError::WriteBytesFailed) => "Failed to write bytes to disk.",
                        Some(UpdateError::ExtractFailed) => "Failed to extract archive.",
                        None => "Unknown error",
                    }),
                ]
                .align_x(Alignment::Center)
                .spacing(20),
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
        }

        if self.complete {
            return container(
                column![
                    text("Update complete").size(24),
                    text("The wavey app has been updated to the latest version."),
                ]
                .align_x(Alignment::Center)
                .spacing(20),
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
        } else {
            container(
                column![
                    text("wavey updater").size(24),
                    text("This will update the wavey app to the latest version."),
                    button("Begin").on_press(Message::BeginPressed),
                ]
                .align_x(Alignment::Center)
                .spacing(20),
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
        }
    }
}

impl Default for Updater {
    fn default() -> Self {
        Self {
            complete: false,
            versions_match: false,
            error: false,
            error_type: None,
        }
    }
}

async fn start_update() -> Result<UpdateState, UpdateError> {
    println!("Starting update");

    let local_version = get_local_version().await;
    let remote_version = get_remote_version().await.unwrap();

    let url = format!(
        "https://github.com/isgood-dev/wavey/releases/download/{}/update-package.7z",
        remote_version
    );

    if local_version == remote_version {
        return Ok(UpdateState::VersionMatch);
    }

    let response = reqwest::get(url)
        .await
        .map_err(|_| UpdateError::ReqwestError)?;

    let bytes = response
        .bytes()
        .await
        .map_err(|_| UpdateError::ReqwestError)?;

    let _ = write_temp_bytes(bytes.to_vec())
        .await
        .map_err(|_| UpdateError::WriteBytesFailed)?;

    println!("Extracting archive");
    extract_archive().await?;

    println!("Copying over files");

    println!("Update complete");

    // if local_version == remote_version {
    //     return Ok(UpdateState::VersionMatch);
    // }

    Ok(UpdateState::Completed)
}

async fn write_temp_bytes(bytes: Vec<u8>) -> Result<(), std::io::Error> {
    let _ = tokio::fs::create_dir_all("./temp")
        .await
        .map_err(|_| UpdateError::IoError);

    println!("Created temp directory");

    println!("Writing bytes");

    tokio::fs::write("./temp/update-package.7z", bytes).await?;

    println!("Done.");

    Ok(())
}

async fn extract_archive() -> Result<(), UpdateError> {
    let output = tokio::process::Command::new("7z")
        .arg("x")
        .arg("./temp/update-package.7z")
        .arg("-o.")
        .output()
        .await
        .map_err(|_| UpdateError::ExtractFailed)?;

    println!("{:?}", output);

    fs::remove_dir_all("./temp")
        .await
        .map_err(|_| UpdateError::IoError)?;

    if output.status.success() {
        Ok(())
    } else {
        Err(UpdateError::ExtractFailed)
    }
}

async fn get_local_version() -> String {
    tokio::fs::read("../VERSION")
        .await
        .map(|v| String::from_utf8_lossy(&v).to_string())
        .unwrap_or_else(|_| "v0.0.0".to_string())
}

async fn get_remote_version() -> Result<String, reqwest::Error> {
    let response =
        reqwest::get("https://raw.githubusercontent.com/isgood-dev/wavey/rust-rewrite/VERSION")
            .await?;

    let text = response.text().await?;

    Ok(text)
}

#[derive(Debug, Clone)]
enum UpdateState {
    VersionMatch,
    Completed,
}

#[derive(Debug, Clone)]
enum UpdateError {
    ReqwestError,
    IoError,
    WriteBytesFailed,
    ExtractFailed,
}
