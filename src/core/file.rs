use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum FileError {
    DialogClosed,
}

pub async fn pick_file() -> Result<PathBuf, FileError> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Select file")
        .pick_file()
        .await
        .ok_or(FileError::DialogClosed)?;

    Ok(handle.path().to_owned())
}
