use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct TimeTableResponse {
    pub id: String,
    pub name: String,
    degrees: Vec<String>,
    year: i32,
    pub acadYear: i32,
    pub sections: Vec<SectionResponse>,
    pub timings: Vec<String>,
    pub examTimes: Vec<String>,
}
#[derive(Deserialize, Debug, Default)]
pub struct SectionResponse {
    pub courseId: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub number: i32,
    pub instructors: Vec<String>,
    pub roomTime: Vec<String>,
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
}
#[derive(Deserialize, Debug, Default)]
pub struct HolidayString {
    pub name: String,
    pub date: String,
}
#[derive(Deserialize, Debug, Default)]
pub struct TimeTableChangeResponse {
    pub date: String,
    pub day: String,
}
#[derive(Deserialize, Debug, Default)]
pub struct HolidayResponse {
    pub classwork_start: String,
    pub classwork_end: String,
    pub midsem_start: String,
    pub midsem_end: String,
    pub holidays: Vec<HolidayString>,
    pub time_table_changes: Vec<TimeTableChangeResponse>,
}
