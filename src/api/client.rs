use crate::api::data::TimeTable;
use crate::api::responses::{CourseResponse, HolidayResponse, TimeTableResponse};
use reqwest::Error;

#[derive(Debug)]
pub struct Client {
    id: String,
    ttr: Option<TimeTableResponse>,
    cr: Option<CourseResponse>,
    holiday_response: Option<HolidayResponse>,
    pub timetable: Option<TimeTable>,
}

impl Client {
    pub async fn new(id: String) -> Result<Self, Error> {
        let mut client = Self {
            id,
            ttr: None,
            cr: None,
            holiday_response: None,
            timetable: None,
        };

        client.fetch_courses().await?;
        client.fetch_timetable().await?;
        client.fetch_holidays().await?;
        client.update_time_table();
        Ok(client)
    }
    // https://raw.githubusercontent.com/lokesh185/chrono-to-ics-prototype/master/holidays.json
    fn update_time_table(&mut self) -> Option<()> {
        self.timetable = TimeTable::new(
            self.ttr.as_ref()?,
            self.cr.as_ref()?,
            self.holiday_response.as_ref()?,
        );
        Some(())
    }
    async fn fetch_holidays(&mut self) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let cresponse = client
            .get("https://raw.githubusercontent.com/lokesh185/chrono-to-ics-prototype/master/holidays.json")
            .send()
            .await
            ?;
        self.holiday_response = match cresponse.status() {
            reqwest::StatusCode::OK => Some(cresponse.json::<HolidayResponse>().await?),
            other => {
                panic!("unknown error {}", other);
            }
        };
        Ok(())
    }
    async fn fetch_courses(&mut self) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let cresponse = client
            .get("https://www.chrono.crux-bphc.com/api/course")
            .send()
            .await?;
        self.cr = match cresponse.status() {
            reqwest::StatusCode::OK => Some(cresponse.json::<CourseResponse>().await?),
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
        let response = client.get(api_link).send().await?;

        self.ttr = Some(
            response
                .error_for_status()?
                .json::<TimeTableResponse>()
                .await?,
        );
        Ok(())
    }
}
