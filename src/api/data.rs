use crate::api::responses;
use iso8601::DateTime;

#[derive(Debug)]
pub struct CourseName {
    pub id: String,
    pub code: String,
    pub name: String,
}
impl CourseName {
    pub fn from_response_course(course: &responses::Course) -> Self {
        Self {
            id: course.id.clone(),
            code: course.code.clone(),
            name: course.name.clone(),
        }
    }
}
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

pub struct Course {
    code: String,
    name: String,
    timings: Vec<Timing>,
}
#[derive(Debug)]
pub struct TimeTable {
    id: String,
    name: String,
    acad_year: String,
}
