use std::fmt;

#[derive(Debug)]
pub struct Clock {
    minutes: i32,
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        if hours * 60 + minutes < 0 {
            return Self {
                minutes: Self::convert_to_positive_minutes(hours * 60 + minutes),
            };
        }

        Self {
            minutes: (hours * 60 + minutes) % (24 * 60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut new_minutes = self.minutes + minutes;

        if new_minutes < 0 {
            new_minutes = Self::convert_to_positive_minutes(new_minutes);
        }

        Self::new(new_minutes / 60, new_minutes % 60)
    }

    pub fn convert_to_positive_minutes(minutes: i32) -> i32 {
        let total_minutes = minutes.abs() as f32;
        ((total_minutes / (24. * 60.)).ceil() * (24. * 60.) - total_minutes) as i32
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            (self.minutes / 60) % 24,
            self.minutes % 60
        )
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes
    }
}

// #[test]
// fn custom_test() {
//     assert_eq!(Clock::new(10, 37), Clock::new(34, 37));
// }
