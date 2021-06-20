use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

// Returns minutes and hours_offset
fn get_minutes_offset(minutes: i32) -> (i32, i32) {
    let mut minutes_saved = minutes;
    let mut hours_offset = 0;
    
    if minutes_saved < 0 {
        if minutes_saved <= -60 {
            hours_offset += minutes_saved / 60;
            minutes_saved %= 60;
        }
        minutes_saved += 60;
        hours_offset -= 1;
    }
    if minutes_saved >= 60 {
        hours_offset += minutes_saved / 60;
        minutes_saved %= 60;
    }

    (minutes_saved, hours_offset)
}

fn get_hours(hours: i32) -> i32 {
    let mut hours_saved = hours;

    if hours_saved < 0 {
        if hours_saved <= -24 {
            hours_saved %= 24;
        }
        hours_saved += 24;
    } 
    if hours_saved >= 24 {
        hours_saved %= 24;
    }

    hours_saved
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (minutes_saved, hours_offset) = get_minutes_offset(minutes);

        Self {
            minutes: minutes_saved,
            hours: get_hours(hours + hours_offset),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (minutes_saved, hours_offset) = get_minutes_offset(self.minutes + minutes);
        let hours_saved = get_hours(self.hours + hours_offset);

        Self {
            minutes: minutes_saved,
            hours: hours_saved,
        }
    }
}

impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

// impl fmt::Debug for Clock {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Clock")
//          .field("hours", &self.hours)
//          .field("minutes", &self.minutes)
//          .finish()
//     }
// }
