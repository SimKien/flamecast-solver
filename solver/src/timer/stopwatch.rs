use super::get_time_diff_pretty;

#[derive(Debug, Clone)]
pub struct Stopwatch {
    pub start: chrono::DateTime<chrono::Utc>,
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch {
            start: chrono::Utc::now(),
        }
    }

    pub fn restart(&mut self) {
        self.start = chrono::Utc::now();
    }

    pub fn elapsed(&self) -> chrono::Duration {
        chrono::Utc::now().signed_duration_since(self.start)
    }

    pub fn elapsed_pretty(&self) -> String {
        get_time_diff_pretty(self.elapsed())
    }
}
