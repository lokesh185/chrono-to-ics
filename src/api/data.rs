use std::{error::Error, str::FromStr};

use super::responses::{CourseResponse, SectionResponse, TimeTableResponse};
use iso8601::{Date, DateTime};
use regex;

#[derive(Debug, PartialEq, Clone)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

#[derive(Debug, Clone)]
pub struct Timing {
    day: Day,
    classroom: String,
    start: u8,
    end: u8,
}

impl Timing {
    pub fn from_string(info: &str) -> Option<Self> {
        let re = regex::Regex::new(r"\w+ \w\d{3}:(\w\d{3}):(\w+):(\d+)").unwrap();

        let Some(caps) = re.captures(&info) else {
            return None;
        };
        // dbg!(&caps);
        // let code = caps.get(1).unwrap().as_str().to_string();
        let classroom = caps.get(1).unwrap().as_str().to_string();
        let day = match caps.get(2).unwrap().as_str() {
            "M" => Day::Monday,
            "T" => Day::Tuesday,
            "W" => Day::Wednesday,
            "Th" => Day::Thursday,
            "F" => Day::Friday,
            _ => {
                return None;
            }
        };
        let time = caps.get(3).unwrap().as_str().parse::<u8>().unwrap();
        Some(Timing {
            day,
            classroom,
            start: time,
            end: time,
        })
    }
}
#[derive(Debug)]
pub struct Section {
    number: i32,
    instructors: Vec<String>,
    timings: Vec<Timing>,
}
impl Section {
    pub fn optimize_timings(&mut self) {
        let mut new_timings: Vec<Timing> = vec![];
        self.timings.iter().for_each(|timing| {
            match new_timings.iter_mut().find(|new_timing| {
                (new_timing.day == timing.day)
                    && ((new_timing.start == timing.end + 1)
                        || (new_timing.end + 1 == timing.start))
            }) {
                Some(new_timing) => {
                    if new_timing.start == timing.end + 1 {
                        new_timing.start -= 1;
                    } else {
                        new_timing.end += 1;
                    }
                }
                None => new_timings.push(timing.clone()),
            }
        });
        self.timings = new_timings;
    }
}
#[derive(Debug, Default)]
pub struct Course {
    id: String,
    code: String,
    name: String,
    pub lecture: Option<Section>,
    pub tutorial: Option<Section>,
    pub lab: Option<Section>,
    pub midsem_date_time: Option<(DateTime, DateTime)>,
    pub compre_date_time: Option<(DateTime, DateTime)>,
}
impl Course {
    fn new(id: String, course_response: &CourseResponse) -> Option<Self> {
        course_response
            .courses
            .iter()
            .find_map(|course| match course.id.eq(&id) {
                true => Some(Self {
                    id: id.clone(),
                    code: course.code.clone(),
                    name: course.name.clone(),
                    lecture: None,
                    tutorial: None,
                    lab: None,
                    midsem_date_time: None,
                    compre_date_time: None,
                }),
                false => None,
            })
    }
    fn add_section(&mut self, section_response: &SectionResponse) {
        let timings = section_response
            .roomTime
            .iter()
            .filter_map(|info| match Timing::from_string(info) {
                Some(timing) => Some(timing),
                None => None,
            })
            .collect::<Vec<Timing>>();
        let mut new_section = Section {
            number: section_response.number,
            instructors: section_response.instructors.clone(),
            timings: timings,
        };
        new_section.optimize_timings();
        match section_response.type_name.as_str() {
            "P" => self.lab = Some(new_section),
            "T" => self.tutorial = Some(new_section),
            "L" => self.lecture = Some(new_section),
            _ => {}
        }
    }
    fn update_exam_time(&mut self, exam_times: &Vec<ExamTime>) {
        exam_times
            .iter()
            .filter_map(|f| match f.code.eq(&self.code) {
                true => Some(f),
                false => None,
            })
            .for_each(|exam_time| match exam_time.exam_type {
                ExamKind::Midsem => {
                    self.midsem_date_time =
                        Some((exam_time.start_date_time, exam_time.end_date_time))
                }
                ExamKind::Compre => {
                    self.compre_date_time =
                        Some((exam_time.start_date_time, exam_time.end_date_time))
                }
            })
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
    pub fn new(time_table_response: &TimeTableResponse, course_response: &CourseResponse) -> Self {
        let mut courses: Vec<Course> = vec![];
        time_table_response
            .sections
            .iter()
            .for_each(|section_response| {
                match courses.iter_mut().find_map(|course| {
                    match course.id.eq(&section_response.courseId) {
                        true => Some(course),
                        false => None,
                    }
                }) {
                    Some(course) => {
                        course.add_section(section_response);
                    }
                    None => {
                        if let Some(mut course) =
                            Course::new(section_response.courseId.clone(), course_response)
                        {
                            course.add_section(section_response);
                            courses.push(course)
                        }
                    }
                };
            });
        let exam_times = time_table_response
            .examTimes
            .iter()
            .filter_map(|info| match ExamTime::from_string(info.clone()) {
                Ok(exam_time) => Some(exam_time),
                Err(_) => None,
            })
            .collect::<Vec<ExamTime>>();

        courses
            .iter_mut()
            .for_each(|course| course.update_exam_time(&exam_times));
        Self {
            id: time_table_response.id.clone(),
            name: time_table_response.name.clone(),
            acad_year: time_table_response.acadYear,
            courses: courses,
        }
    }
}
pub enum ExamKind {
    Midsem,
    Compre,
}
pub struct ExamTime {
    code: String,
    exam_type: ExamKind,
    start_date_time: DateTime,
    end_date_time: DateTime,
}
impl ExamTime {
    fn from_string(info: String) -> Result<Self, regex::Error> {
        let re = regex::Regex::new(r"(\w+ \w\d{3})\|(\w{6})\|([^\|]+)\|(.+)")?;

        let Some(caps) = re.captures(&info) else {
            return Err(regex::Error::Syntax(
                "required values not found in string".to_string(),
            ));
        };
        dbg!(&caps);
        Ok(Self {
            code: match caps.get(1) {
                Some(capture) => capture,
                None => {
                    return Err(regex::Error::Syntax("code not present".to_string()));
                }
            }
            .as_str()
            .to_string(),
            exam_type: match match caps.get(2) {
                Some(capture) => capture,
                None => {
                    return Err(regex::Error::Syntax("exam kind not present".to_string()));
                }
            }
            .as_str()
            {
                "MIDSEM" => ExamKind::Midsem,
                "COMPRE" => ExamKind::Compre,
                _ => {
                    return Err(regex::Error::Syntax("exam type not vaild".to_string()));
                }
            },
            start_date_time: match caps.get(3) {
                Some(capture) => match DateTime::from_str(capture.as_str()) {
                    Ok(date_time) => date_time,
                    Err(_) => {
                        return Err(regex::Error::Syntax(
                            "start time syntax is incorrect".to_string(),
                        ));
                    }
                },
                None => {
                    return Err(regex::Error::Syntax("start time not present".to_string()));
                }
            },
            end_date_time: match caps.get(4) {
                Some(capture) => match DateTime::from_str(capture.as_str()) {
                    Ok(date_time) => date_time,
                    Err(_) => {
                        return Err(regex::Error::Syntax(
                            "end time syntax is incorrect".to_string(),
                        ));
                    }
                },
                None => {
                    return Err(regex::Error::Syntax("end time not present".to_string()));
                }
            },
        })
    }
}
