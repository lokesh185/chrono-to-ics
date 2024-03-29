use super::responses::{CourseResponse, HolidayResponse, SectionResponse, TimeTableResponse};
use chrono::{DateTime, Utc, Weekday};
use regex;
use std::{fmt, str::FromStr, vec};
#[derive(Debug, Clone)]
struct DayError;
impl fmt::Display for DayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid string to parse to Weekday")
    }
}
#[derive(Debug)]
struct WeekdayWrapper(Weekday);
impl WeekdayWrapper {
    pub fn consume_to_weekday(self) -> Weekday {
        self.0
    }
}
impl FromStr for WeekdayWrapper {
    type Err = DayError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WeekdayWrapper(match s {
            "M" => Weekday::Mon,
            "T" => Weekday::Tue,
            "W" => Weekday::Wed,
            "Th" => Weekday::Thu,
            "F" => Weekday::Fri,
            _ => {
                return Err(DayError);
            }
        }))
    }
}
#[derive(Debug, Clone)]
pub struct Timing {
    pub day: Weekday,
    pub classroom: String,
    pub start: u8,
    pub end: u8,
}

impl Timing {
    pub fn from_string(info: &str) -> Option<Self> {
        let re = regex::Regex::new(r"\w+ \w\d{3}:(\w\d{3}):(\w+):(\d+)").ok()?;

        let Some(caps) = re.captures(info) else {
            return None;
        };
        let classroom = caps.get(1)?.as_str().to_string();
        let day = caps
            .get(2)?
            .as_str()
            .parse::<WeekdayWrapper>()
            .ok()?
            .consume_to_weekday();
        let time = caps.get(3)?.as_str().parse::<u8>().ok()?;
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
    pub number: i32,
    pub instructors: Vec<String>,
    pub timings: Vec<Timing>,
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
    pub code: String,
    pub name: String,
    pub lecture: Option<Section>,
    pub tutorial: Option<Section>,
    pub lab: Option<Section>,
    pub midsem_date_time: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub compre_date_time: Option<(DateTime<Utc>, DateTime<Utc>)>,
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
            .filter_map(|info| Timing::from_string(info))
            .collect::<Vec<Timing>>();
        let mut new_section = Section {
            number: section_response.number,
            instructors: section_response.instructors.clone(),
            timings,
        };
        new_section.optimize_timings();
        match section_response.type_name.as_str() {
            "P" => self.lab = Some(new_section),
            "T" => self.tutorial = Some(new_section),
            "L" => self.lecture = Some(new_section),
            _ => {}
        };
    }
    fn update_exam_time(&mut self, exam_times: &[ExamTime]) {
        exam_times
            .iter()
            .filter(|exam_time| exam_time.code.eq(&self.code))
            .for_each(|exam_time| match exam_time.exam_type {
                ExamKind::Midsem => {
                    self.midsem_date_time =
                        Some((exam_time.start_date_time, exam_time.end_date_time))
                }
                ExamKind::Compre => {
                    self.compre_date_time =
                        Some((exam_time.start_date_time, exam_time.end_date_time))
                }
            });
    }
}
#[derive(Debug, Default)]
pub struct Holiday {
    pub name: String,
    pub date: DateTime<Utc>,
}
#[derive(Debug)]
struct TimeTableChange {
    day: Weekday,
    date: DateTime<Utc>,
}
#[derive(Debug)]
pub struct TimeTable {
    pub id: String,
    pub name: String,
    pub acad_year: i32,
    pub classwork_start: DateTime<Utc>,
    pub classwork_end: DateTime<Utc>,
    pub midsem_dates: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub courses: Vec<Course>,
    pub holidays: Vec<Holiday>,
    time_table_changes: Vec<TimeTableChange>,
}
impl TimeTable {
    pub fn new(
        time_table_response: &TimeTableResponse,
        course_response: &CourseResponse,
        holiday_response: &HolidayResponse,
    ) -> Option<Self> {
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
            .filter_map(|info| ExamTime::from_string(info.clone()).ok())
            .collect::<Vec<ExamTime>>();
        dbg!(&courses);

        courses
            .iter_mut()
            .for_each(|course| course.update_exam_time(&exam_times));
        dbg!(&courses);

        let holidays = holiday_response
            .holidays
            .iter()
            .filter_map(|holiday_string| {
                Some(Holiday {
                    name: holiday_string.name.clone(),
                    date: holiday_string.date.as_str().parse::<DateTime<Utc>>().ok()?,
                })
            })
            .collect::<Vec<Holiday>>();
        dbg!(&holidays);
        let time_table_changes = holiday_response
            .time_table_changes
            .iter()
            .filter_map(|ttcr| {
                Some(TimeTableChange {
                    date: ttcr.date.parse::<DateTime<Utc>>().ok()?,
                    day: ttcr
                        .day
                        .parse::<WeekdayWrapper>()
                        .ok()?
                        .consume_to_weekday(),
                })
            })
            .collect::<Vec<TimeTableChange>>();
        Some(Self {
            id: time_table_response.id.clone(),
            name: time_table_response.name.clone(),
            acad_year: time_table_response.acadYear,
            courses,
            holidays,
            time_table_changes,
            midsem_dates: if let (Ok(midsem_st), Ok(midsem_end)) = (
                holiday_response.midsem_start.parse::<DateTime<Utc>>(),
                holiday_response.midsem_end.parse::<DateTime<Utc>>(),
            ) {
                Some((midsem_st, midsem_end))
            } else {
                None
            },
            classwork_start: holiday_response
                .classwork_start
                .parse::<DateTime<Utc>>()
                .ok()?,
            classwork_end: holiday_response
                .classwork_end
                .parse::<DateTime<Utc>>()
                .ok()?,
        })
    }
}
pub enum ExamKind {
    Midsem,
    Compre,
}
pub struct ExamTime {
    code: String,
    exam_type: ExamKind,
    start_date_time: DateTime<Utc>,
    end_date_time: DateTime<Utc>,
}
impl ExamTime {
    fn from_string(info: String) -> Result<Self, regex::Error> {
        let re = regex::Regex::new(r"(\w+ \w\d{3})\|(\w{6})\|([^\|]+)\|(.+)")?;

        let caps = re.captures(&info).ok_or_else(|| {
            regex::Error::Syntax("required values not found in string".to_string())
        })?;
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
                Some(capture) => match capture.as_str().parse::<DateTime<Utc>>() {
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
                Some(capture) => match capture.as_str().parse::<DateTime<Utc>>() {
                    Ok(date_time) => date_time,
                    Err(_) => {
                        return Err(regex::Error::Syntax(
                            "start time syntax is incorrect".to_string(),
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
