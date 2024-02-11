use crate::api::data::TimeTable;
use crate::api::responses::{CourseResponse, HolidayResponse, TimeTableResponse};
use reqwest::Error;

#[derive(Debug)]
pub struct Client {
    id: String,
    pub ttr: TimeTableResponse,
    cr: CourseResponse,
    holiday_response: HolidayResponse,
    pub timetable: Option<TimeTable>,
    // time_table: data::TimeTable,
}

impl Client {
    pub async fn new(id: String) -> Result<Self, Error> {
        let mut client = Self {
            id,
            ttr: TimeTableResponse::default(),
            cr: CourseResponse::default(),
            holiday_response: HolidayResponse::default(),
            timetable: None,
        };

        client.fetch_courses().await.unwrap();
        client.fetch_timetable().await.unwrap();
        client.fetch_holidays().await.unwrap();
        client.update_time_table();
        Ok(client)
    }
    // https://raw.githubusercontent.com/lokesh185/chrono-to-ics-prototype/master/holidays.json
    fn update_time_table(&mut self) {
        self.timetable = TimeTable::new(&self.ttr, &self.cr, &self.holiday_response);
    }
    async fn fetch_holidays(&mut self) -> Result<(), Error> {
        let client = reqwest::Client::new();
        let cresponse = client
            .get("https://raw.githubusercontent.com/lokesh185/chrono-to-ics-prototype/master/holidays.json")
            .send()
            .await
            .unwrap();
        // dbg!(&cresponse);
        self.holiday_response = match cresponse.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponseoptimize_timings
                cresponse.json::<HolidayResponse>().await?
            }
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
            .await
            .unwrap();
        // dbg!(&cresponse);
        self.cr = match cresponse.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponseoptimize_timings
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
    // pub fn write_to_ics_file(&self) -> String {
    //     let time_table = &self.timetable.unwrap();
    //     let mut calendar = Calendar::new();
    //     calendar.append_property(Property::new("X-WR-CALNAME", time_table.name.as_str()));

    //     for course in &time_table.courses {
    //         if let Some(lecture) = &course.lecture {
    //             let lecture_event = create_event(course, lecture, time_table);
    //             calendar.push(lecture_event);
    //         }
    //         if let Some(tutorial) = &course.tutorial {
    //             let tutorial_event = create_event(course, tutorial, time_table);
    //             calendar.push(tutorial_event);
    //         }
    //         if let Some(lab) = &course.lab {
    //             let lab_event = create_event(course, lab, time_table);
    //             calendar.push(lab_event);
    //         }
    //     }

    //     calendar.to_string()
    // }
}

// fn create_event(course: &Course, section: &Section, time_table: &TimeTable) -> Event {
//     let mut event = Event::new();
//     event.append_property(Property::new(
//         "SUMMARY",
//         &format!("{} - {}", &course.code, section.number),
//     ));

//     event.append_property(Property::new("DTSTART", &time_table.classwork_start.to_string()));
//     event.append_property(Property::new("DTEND", &time_table.classwork_end.to_string()));

//     // Add recurrence rule for each day
//     for timing in &section.timings {
//         let day_name = match timing.day {
//             Weekday::Mon => "MO",
//             Weekday::Tue => "TU",
//             Weekday::Wed => "WE",
//             Weekday::Thu => "TH",
//             Weekday::Fri => "FR",
//             Weekday::Sat => "SA",
//             Weekday::Sun => "SU",
//         };
//         let rrule = format!("RRULE:FREQ=WEEKLY;BYDAY={}", day_name);
//     }
//     event.append_property(Property::new("RRULE", rrule));

//     event
// }
