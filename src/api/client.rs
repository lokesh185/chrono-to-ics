use crate::api::data::TimeTable;
use crate::api::responses::{CourseResponse, HolidayResponse, TimeTableResponse};
use reqwest::blocking::Client;
use reqwest::Error;
#[derive(Debug)]
pub struct ApiClient {
    pub id: String,
    ttr: Option<TimeTableResponse>,
    cr: Option<CourseResponse>,
    holiday_response: Option<HolidayResponse>,
    pub timetable: Option<TimeTable>,
}

impl ApiClient {
    pub fn new(id: String) -> Result<Self, Error> {
        let mut client = Self {
            id,
            ttr: None,
            cr: None,
            holiday_response: None,
            timetable: None,
        };

        client.fetch_courses()?;
        client.fetch_holidays()?;
        // client.fetch_timetable()?;
        // client.update_time_table();
        Ok(client)
    }
    // https://raw.githubusercontent.com/lokesh185/chrono-to-ics-prototype/master/holidays.json
    pub fn update_time_table(&mut self) -> Option<()> {
        self.timetable = TimeTable::new(
            self.ttr.as_ref()?,
            self.cr.as_ref()?,
            self.holiday_response.as_ref()?,
        );
        Some(())
    }
    fn fetch_holidays(&mut self) -> Result<(), Error> {
        let client = Client::new();
        let cresponse = client
            .get("https://raw.githubusercontent.com/lokesh185/chrono-to-ics-prototype/master/holidays.json")
            .send()?;
        self.holiday_response = Some(cresponse.error_for_status()?.json::<HolidayResponse>()?);
        Ok(())
    }
    fn fetch_courses(&mut self) -> Result<(), Error> {
        let client = Client::new();
        let cresponse = client
            .get("https://www.chrono.crux-bphc.com/api/course")
            .send()?;
        self.cr = Some(cresponse.error_for_status()?.json::<CourseResponse>()?);
        Ok(())
    }
    pub fn fetch_timetable(&mut self) -> Result<(), Error> {
        let client = Client::new();
        let api_link = format!(
            "https://www.chrono.crux-bphc.com/api/timetable/{}",
            &self.id
        );
        let response = client.get(api_link).send()?;

        self.ttr = Some(response.error_for_status()?.json::<TimeTableResponse>()?);
        Ok(())
    }
}
