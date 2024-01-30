use futures::future::ok;
use iso8601::Date;
use reqwest::Error;
use serde::Deserialize;
// use time::format_description::well_known::{iso8601, Iso8601};
#[derive(Debug)]
pub struct Client {
    id: String,
    ttr: Responses::TimeTableResponse,
    cr: Responses::CourseResponse,
    pub courses: Vec<Data::Course>,
    // time_table: Data::TimeTable,
}

impl Client {
    pub async fn new(id: String) -> Result<Self, Error> {
        let api_link = format!("https://www.chrono.crux-bphc.com/api/timetable/{}", &id);
        let client = reqwest::Client::new();
        let response = client.get(api_link).send().await.unwrap();

        let timetableresponse = match response.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponse
                response.json::<Responses::TimeTableResponse>().await?
            }
            other => {
                panic!("unknown error {}", other);
            }
        };
        dbg!(&timetableresponse);
        let client = reqwest::Client::new();

        let cresponse = client
            .get("https://www.chrono.crux-bphc.com/api/course")
            .send()
            .await
            .unwrap();
        // dbg!(&cresponse);
        let course_response = match cresponse.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponse
                cresponse.json::<Responses::CourseResponse>().await?
            }
            other => {
                panic!("unknown error {}", other);
            }
        };
        let mut clinet = Self {
            id,
            ttr: timetableresponse,
            cr: course_response,
            courses: vec![],
        };
        clinet.parse_courses();
        // dbg!(&timetableresponse, &courseresponse);
        Ok(clinet)
    }
    fn parse_courses(&mut self) {
        self.courses = self
            .cr
            .courses
            .iter()
            .map(|course| match Data::Course::from_response_course(course) {
                Ok(course_data) => course_data,
                Err(_) => Data::Course::from_response_course_without_date_time(course),
            })
            .collect()
    }
}
mod Responses {
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

pub mod Data {
    use super::Responses;
    use iso8601::{DateTime, Time};
    #[derive(Debug)]
    pub struct TimeTable {}
    #[derive(Debug)]
    pub struct Course {
        id: String,
        code: String,
        name: String,
        midsem_time: Option<(DateTime, DateTime)>,
        compre_time: Option<(DateTime, DateTime)>,
    }
    impl Course {
        pub fn from_response_course(course: &Responses::Course) -> Result<Self, String> {
            let compre_time: Option<(DateTime, DateTime)> = if let (Some(start), Some(end)) =
                (&course.compreStartTime, &course.compreEndTime)
            {
                Some((iso8601::datetime(start)?, iso8601::datetime(end)?))
            } else {
                None
            };
            let midsem_time: Option<(DateTime, DateTime)> = if let (Some(start), Some(end)) =
                (&course.compreStartTime, &course.compreEndTime)
            {
                Some((iso8601::datetime(start)?, iso8601::datetime(end)?))
            } else {
                None
            };
            Ok(Self {
                id: course.id.clone(),
                code: course.code.clone(),
                name: course.name.clone(),
                midsem_time,
                compre_time,
            })
        }
        pub fn from_response_course_without_date_time(course: &Responses::Course) -> Self {
            Self {
                id: course.id.clone(),
                code: course.code.clone(),
                name: course.name.clone(),
                midsem_time: None,
                compre_time: None,
            }
        }
    }
}
