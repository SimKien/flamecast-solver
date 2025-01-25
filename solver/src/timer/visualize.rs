use chrono::TimeDelta;

pub fn get_time_diff_pretty(diff: TimeDelta) -> String {
    let num_millis = diff.num_milliseconds() % 1_000;
    let num_seconds = diff.num_seconds() % 60;
    let num_minutes = (diff.num_seconds() / 60) % 60;
    let num_hours = diff.num_seconds() / 3600;
    let result = if diff.num_nanoseconds().is_some() {
        let num_nano = diff.num_nanoseconds().unwrap() % 1_000;
        let num_micro = diff.num_microseconds().unwrap() % 1_000;
        format!(
            "{}h {}min {}s {}ms {}us {}ns",
            num_hours, num_minutes, num_seconds, num_millis, num_micro, num_nano
        )
    } else if diff.num_microseconds().is_some() {
        let num_micro = diff.num_microseconds().unwrap() % 1_000;
        format!(
            "{}h {}min {}s {}ms {}us",
            num_hours, num_minutes, num_seconds, num_millis, num_micro
        )
    } else {
        format!(
            "{}h {}min {}s {}ms",
            num_hours, num_minutes, num_seconds, num_millis
        )
    };
    return result;
}
