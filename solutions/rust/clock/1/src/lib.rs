#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * 60 + minutes).rem_euclid(24 * 60);
        Self { minutes: total_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        format!("{:02}:{:02}", hours, minutes)
    }
}
