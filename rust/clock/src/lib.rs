#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let fixed_hours = if hours < 0 { 24 + (hours % 24) } else { hours };
        Clock {
            hours: fixed_hours,
            minutes: 0,
        }
        .add_minutes(minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes_sum = self.minutes + minutes;
        let hours_to_add = if minutes_sum < 0 {
            (minutes_sum / 60) - 1
        } else {
            minutes_sum / 60
        };

        let raw_minutes = minutes_sum - (hours_to_add * 60);

        let minutes = if raw_minutes < 0 {
            60 + (raw_minutes % 60)
        } else {
            raw_minutes
        };

        let mut hours = self.hours + hours_to_add;
        hours = if raw_minutes >= 60 {
            hours + raw_minutes / 60
        } else {
            hours
        };
        hours = if hours < 0 {
            24 + (hours % 24)
        } else {
            hours
        };

        Clock { hours: hours % 24, minutes: minutes % 60}
    }
}

use std::string::ToString;

impl ToString for Clock {
    fn to_string(&self) -> String {
        let hours_prefix = if self.hours <= 9 { "0" } else { "" };
        let hours = format!("{}{}", hours_prefix, self.hours);

        let minutes_prefix = if self.minutes <= 9 { "0" } else { "" };
        let minutes = format!("{}{}", minutes_prefix, self.minutes);

        return format!("{}:{}", hours, minutes);
    }
}
