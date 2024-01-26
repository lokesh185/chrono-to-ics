use reqwest::Error;
use serde::Deserialize;
// use time::format_description::well_known::{iso8601, Iso8601};
#[derive(Debug)]
pub struct Data {
    id: String,
    ttr: TimeTableResponse,
    cr: CourseResponse,
}

impl Data {
    pub async fn new(id: String) -> Result<Data, Error> {
        let api_link = format!("https://www.chrono.crux-bphc.com/api/timetable/{}", &id);
        let client = reqwest::Client::new();
        let response = client.get(api_link).send().await.unwrap();

        let timetableresponse = match response.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponse
                response.json::<TimeTableResponse>().await?
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
        let courseresponse = match cresponse.status() {
            reqwest::StatusCode::OK => {
                // on success, parse our JSON to an APIResponse
                cresponse.json::<CourseResponse>().await?
            }
            other => {
                panic!("unknown error {}", other);
            }
        };
        // dbg!(&timetableresponse, &courseresponse);
        Ok(Data {
            id,
            ttr: timetableresponse,
            cr: courseresponse,
        })
    }
    async fn fetch_courses(&mut self) -> Result<&mut Self, ()> {
        reqwest::Client::new()
            .get("https://www.chrono.crux-bphc.com/api/course")
            .send()
            .await
            .unwrap()
            .status();
        Ok(self)
    }
}
#[derive(Deserialize, Debug, Default)]
struct TimeTableResponse {
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
struct Section {
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

struct CourseResponse {
    courses: Vec<Course>,
}
#[derive(Deserialize, Debug, Default)]

struct Course {
    id: String,
    code: String,
    name: String,
    // time uses this format yyyy-MM-dd'T'HH:mm:ss.SSS'Z'
    midesmStartTime: Option<String>,
    midesmEndTime: Option<String>,
    compreStartTime: Option<String>,
    compreEndTime: Option<String>,
    archived: bool,
    acadYear: i32,
    semester: i32,
    createdAt: String,
}
