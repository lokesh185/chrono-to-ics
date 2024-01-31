use super::responses::{CourseResponse, SectionResponse, TimeTableResponse};
use iso8601::DateTime;
use regex;
#[derive(Debug)]
pub enum TimingError {
    BadStringFormat,
    InvalidTimingFormat,
    InvalidDay,
    InvalidTime,
}

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

        Self {
            id: time_table_response.id.clone(),
            name: time_table_response.name.clone(),
            acad_year: time_table_response.acadYear,
            courses: courses,
        }
    }
}
