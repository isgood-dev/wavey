// Formats a duration in seconds to a human-readable format.
pub fn format_duration(seconds: u64) -> String {
    let minutes = seconds / 60;
    let remaining_minutes = seconds % 60;

    format!("{}:{:02}", minutes, remaining_minutes)
}

// Truncates a track name and adds an ellipsis if name exceeds 30 characters.
pub fn trunc_name(name: &str) -> String {
    if name.len() > 30 {
        format!("{}...", &name[..30])
    } else {
        name.to_string()
    }
}
