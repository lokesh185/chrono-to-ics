use crate::api::responses::{CourseResponse, TimeTableResponse};

use reqwest::Error;
// use time::format_description::well_known::{iso8601, Iso8601};
#[derive(Debug)]
pub struct Client {
    id: String,
    pub ttr: responses::TimeTableResponse,
    cr: responses::CourseResponse,
    pub courses: Vec<data::CourseName>,
    // time_table: data::TimeTable,
}

impl Client {
    pub async fn new(id: String) -> Result<Self, Error> {
        let mut client = Self {
            id,
            ttr: TimeTableResponse::default(),
            cr: CourseResponse::default(),
            courses: vec![],
        };
        client.parse_courses();
        client.fetch_courses().await.unwrap();
        client.fetch_timetable().await.unwrap();
        Ok(client)
    }

    fn parse_courses(&mut self) {
        self.courses = self
            .cr
            .courses
            .iter()
            .map(|course| data::CourseName::from_response_course(course))
            .collect()
    }
    async fn fetch_courses(&mut self) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let cresponse = client
            .get("https://www.chrono.crux-bphc.com/api/course")
            .send()
            .await
            .unwrap();
        // dbg!(&cresponse);
        self.cr = match cresponse.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponse
                cresponse.json::<responses::CourseResponse>().await?
            }
            other => {
                panic!("unknown error {}", other);
            }
        };
        Ok(())
    }
    async fn fetch_timetable(&mut self) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let api_link = format!(
            "https://www.chrono.crux-bphc.com/api/timetable/{}",
            &self.id
        );
        let response = client.get(api_link).send().await.unwrap();

        self.ttr = match response.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponse
                response.json::<responses::TimeTableResponse>().await?
            }
            other => {
                panic!("unknown error {}", other);
            }
        };
        Ok(())
    }
}
#[allow(non_snake_case)]
#[allow(dead_code)]
mod responses {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Default)]
    pub struct TimeTableResponse {
        id: String,
        authorId: String,
        name: String,
        degrees: Vec<String>,
        private: bool,
        draft: bool,
        archived: bool,
        year: i32,
        acadYear: i32,
        timings: Vec<String>,
        examTimes: Vec<String>,
        warnings: Vec<String>,
        createdAt: String,
        lastUpdated: String,
    }
    #[derive(Deserialize, Debug, Default)]
    pub struct Section {
        id: String,
        courseId: String,
        #[serde(rename = "type")]
        type_name: String,
        instructors: Vec<String>,
        roomTime: Vec<String>,
        createdAt: String,
    }
    #[derive(Deserialize, Debug, Default)]
    #[serde(transparent)]
    pub struct CourseResponse {
        pub courses: Vec<Course>,
    }

    #[derive(Deserialize, Debug, Default)]

    pub struct Course {
        pub id: String,
        pub code: String,
        pub name: String,
        // time uses this format yyyy-MM-dd'T'HH:mm:ss.SSS'Z'
        pub midesmStartTime: Option<String>,
        pub midesmEndTime: Option<String>,
        pub compreStartTime: Option<String>,
        pub compreEndTime: Option<String>,
        archived: bool,
        acadYear: i32,
        semester: i32,
        createdAt: String,
    }
}

pub mod data {
    use super::responses;
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
                if let (Some(char1), Some(char2)) =
                    (timing_info_iter.next(), timing_info_iter.next())
                {
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
}
