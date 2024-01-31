use crate::api::data::{Course, CourseName, TimeTable};
use crate::api::responses::{CourseResponse, TimeTableResponse};
use reqwest::Error;
pub struct Client {
    id: String,
    pub ttr: TimeTableResponse,
    cr: CourseResponse,
    pub courses: Vec<CourseName>,
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
            .map(|course| CourseName::from_response_course(course))
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
                cresponse.json::<CourseResponse>().await?
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
                response.json::<TimeTableResponse>().await?
            }
            other => {
                panic!("unknown error {}", other);
            }
        };
        Ok(())
    }
}
