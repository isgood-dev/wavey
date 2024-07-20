// Formats a duration in seconds to a human-readable format.
pub fn duration(seconds: u64) -> String {
    let minutes = seconds / 60;
    let remaining_minutes = seconds % 60;

    format!("{}:{:02}", minutes, remaining_minutes)
}
