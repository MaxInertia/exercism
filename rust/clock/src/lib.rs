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

        let mut hours_to_add = minutes_sum / 60;
        let mut minutes = minutes_sum - (hours_to_add * 60);

        if minutes >= 60 {
            hours_to_add += minutes / 60;
            minutes = minutes % 60;
        } else if minutes < 0 {
            hours_to_add += (minutes / 60) - 1;
            minutes = 60 + (minutes % 60);
        }

        let mut hours = self.hours + hours_to_add;
        hours = if hours < 0 { 24 + (hours % 24) } else { hours };

        Clock {
            hours: hours % 24,
            minutes: minutes,
        }
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
