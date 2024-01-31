use crate::api::data::{Course, TimeTable};
use crate::api::responses::{CourseResponse, TimeTableResponse};
use reqwest::Error;
#[derive(Debug)]
pub struct Client {
    id: String,
    pub ttr: TimeTableResponse,
    cr: CourseResponse,
    timetable: TimeTable,
    // time_table: data::TimeTable,
}

impl Client {
    pub async fn new(id: String) -> Result<Self, Error> {
        let mut client = Self {
            id,
            ttr: TimeTableResponse::default(),
            cr: CourseResponse::default(),
            timetable: TimeTable::default(),
        };

        client.fetch_courses().await.unwrap();
        client.fetch_timetable().await.unwrap();
        client.update_time_table();
        Ok(client)
    }
    fn update_time_table(&mut self) {
        self.timetable = TimeTable::new(&self.ttr);
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
