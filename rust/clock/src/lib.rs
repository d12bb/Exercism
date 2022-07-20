use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock = Clock { hours: 0, minutes: 0, };
        clock.add(hours, minutes)
    }
    
    pub fn add_minutes(&self, minutes: i32) -> Self {
        self.add(0, minutes)
    }

    fn add(&self, hours: i32, minutes: i32) -> Self {
        let mut clock = Clock {
            hours: self.hours + hours,
            minutes: self.minutes + minutes,
        };

        if clock.minutes < 0 {
            clock.hours -= clock.minutes.abs() / 60 + 1;
            clock.minutes = 60 - clock.minutes.abs() % 60;
        }
        if clock.hours < 0 {
            clock.hours = 24 - clock.hours.abs() % 24;
        }

        if clock.minutes >= 60 {
            clock.hours += clock.minutes / 60;
            clock.minutes = clock.minutes % 60;
        }
        if clock.hours >= 24 {
            clock.hours = clock.hours % 24;
        }

        clock
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
