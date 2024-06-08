// Formats a duration in seconds to a human-readable format.
pub fn format_duration(seconds: u64) -> String {
    let minutes = seconds / 60;
    let remaining_minutes = seconds % 60;

    format!("{}:{:02}", minutes, remaining_minutes)
}

// Truncates a track name and adds an ellipsis if name exceeds 30 characters.
pub fn trunc_name(name: &str) -> String {
    if name.chars().count() > 40 {
        let truncated: String = name.chars().take(40).collect();
        format!("{}...", truncated)
    } else {
        name.to_string()
    }
}
