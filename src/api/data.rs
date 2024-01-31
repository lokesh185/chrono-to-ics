use super::responses::TimeTableResponse;
use clap::builder::Str;
use iso8601::DateTime;
#[derive(Debug)]
pub enum TimingError {
    BadStringFormat,
    InvalidTimingFormat,
    InvalidDay,
    InvalidTime,
}

#[derive(Debug)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

#[derive(Debug)]
pub struct Timing {
    code: String,
    day: Day,
    start: u8,
    end: u8,
}

impl Timing {
    pub fn from_string(info: &str) -> Result<Self, TimingError> {
        let mut parts_iter = info.split(':').collect::<Vec<&str>>().into_iter();
        if let (Some(code), Some(timing_info)) = (parts_iter.next(), parts_iter.next()) {
            let mut timing_info_iter = timing_info.chars();
            if let (Some(char1), Some(char2)) = (timing_info_iter.next(), timing_info_iter.next()) {
                let day = match char1 {
                    'M' => Day::Monday,
                    'W' => Day::Wednesday,
                    'T' => match char2 {
                        'h' => Day::Thursday,
                        _ => Day::Tuesday,
                    },
                    'F' => Day::Friday,
                    _ => return Err(TimingError::InvalidDay),
                };
                let optional_char3 = timing_info_iter.next();
                let time_string = match day {
                    Day::Thursday => match optional_char3 {
                        Some(char3) => {
                            if let Some(char4) = timing_info_iter.next() {
                                format!("{}{}", char3, char4)
                            } else {
                                char3.to_string()
                            }
                        }
                        None => {
                            return Err(TimingError::InvalidTime);
                        }
                    },
                    _ => match optional_char3 {
                        Some(char3) => format!("{}{}", char2, char3),
                        None => char2.to_string(),
                    },
                };
                let time = match time_string.parse::<u8>() {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(TimingError::InvalidTime);
                    }
                };
                Ok(Self {
                    code: code.to_string(),
                    day,
                    start: time,
                    end: time,
                })
            } else {
                return Err(TimingError::InvalidTimingFormat);
            }
        } else {
            Err(TimingError::BadStringFormat)
        }
    }
}
#[derive(Debug, Default)]
pub struct Course {
    code: String,
    name: Option<String>,
    pub timings: Vec<Timing>,
    pub midsem_date_time: Option<(DateTime, DateTime)>,
    pub compre_date_time: Option<(DateTime, DateTime)>,
}
impl Course {
    fn push_timing(&mut self, timing: Timing) {
        self.timings.push(timing)
    }
    fn from_timing(timing: Timing) -> Self {
        Self {
            code: timing.code.clone(),
            name: None,
            timings: vec![timing],
            midsem_date_time: None,
            compre_date_time: None,
        }
    }
    fn optimize_timings(&mut self) {
        for timing in &self.timings {
            for timing in &self.timings {}
        }
    }
}

#[derive(Debug, Default)]
pub struct TimeTable {
    id: String,
    name: String,
    acad_year: i32,
    courses: Vec<Course>,
}
impl TimeTable {
    pub fn new(time_table_response: &TimeTableResponse) -> Self {
        let mut timings = time_table_response
            .timings
            .iter()
            .filter_map(|time_info| match Timing::from_string(time_info) {
                Ok(timing) => Some(timing),
                Err(_) => None,
            })
            .collect::<Vec<Timing>>();
        let mut courses: Vec<Course> = vec![];
        timings.into_iter().for_each(|timing| {
            dbg!(&timing);
            match courses
                .iter_mut()
                .find_map(|course| match course.code.eq(&timing.code) {
                    true => Some(course),
                    false => None,
                }) {
                Some(course) => course.push_timing(timing),

                None => courses.push(Course::from_timing(timing)),
            }
        });
        Self {
            id: time_table_response.id.clone(),
            name: time_table_response.name.clone(),
            acad_year: time_table_response.acadYear,
            courses: courses,
        }
    }
}
