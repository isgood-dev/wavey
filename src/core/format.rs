// Formats a duration in seconds to a human-readable format.
pub fn format_duration(seconds: u64) -> String {
    let minutes = seconds / 60;
    let remaining_minutes = seconds % 60;

    format!("{}:{:02}", minutes, remaining_minutes)
}

// Interpolates the total seconds of a track to a percentage value for use on the progress bar slider.
pub fn interpolate_seconds_to_slider(seconds: u64, total: u64) -> f32 {
    (seconds as f32 / total as f32) * 100.0
}