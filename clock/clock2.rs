use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: Clock::normalize(hours * 60 + minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: Clock::normalize(self.minutes + minutes),
        }
    }

    fn normalize(val: i32) -> i32 {
        let mut val = val;
        while val < 0 {
            val += 1440;
        }
        val % 1440
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours: i32 = self.minutes / 60;
        let minutes: i32 = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
