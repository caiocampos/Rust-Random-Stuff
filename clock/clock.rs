use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (h, m) = Clock::calc(hours, minutes);
        Clock {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    fn calc(hours: i32, minutes: i32) -> (i32, i32) {
        let (h, m) = Clock::normalize(60, minutes);
        let (_, h) = Clock::normalize(24, h + hours);
        (h, m)
    }

    fn normalize(n: i32, val: i32) -> (i32, i32) {
        let mut c = 0;
        let mut val = val;
        while val < 0 {
            c -= 1;
            val += n;
        }
        while val >= n {
            c += 1;
            val -= n;
        }
        (c, val)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl From<String> for Clock {
    fn from(s: String) -> Self {
        let numbers: Vec<i32> = s.split(is_not_numeric).map(parse_number).collect();
        Clock::new(get(&numbers, 0), get(&numbers, 1))
    }
}

impl From<&str> for Clock {
    fn from(s: &str) -> Self {
        Clock::from(s.to_owned())
    }
}

impl From<Clock> for String {
    fn from(c: Clock) -> Self {
        c.to_string()
    }
}

fn is_not_numeric(c: char) -> bool {
    !c.is_numeric()
}

fn parse_number(s: &str) -> i32 {
    s.parse().unwrap_or(0)
}

fn get(vec: &Vec<i32>, pos: usize) -> i32 {
    *vec.get(pos).unwrap_or(&0)
}