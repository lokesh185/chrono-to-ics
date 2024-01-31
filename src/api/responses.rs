use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct TimeTableResponse {
    pub id: String,
    authorId: String,
    pub name: String,
    degrees: Vec<String>,
    private: bool,
    draft: bool,
    archived: bool,
    year: i32,
    pub acadYear: i32,
    pub sections: Vec<SectionResponse>,
    pub timings: Vec<String>,
    pub examTimes: Vec<String>,
    warnings: Vec<String>,
    createdAt: String,
    lastUpdated: String,
}
#[derive(Deserialize, Debug, Default)]
pub struct SectionResponse {
    id: String,
    pub courseId: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub number: i32,
    pub instructors: Vec<String>,
    pub roomTime: Vec<String>,
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
}
