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
    pub timings: Vec<String>,
    pub examTimes: Vec<String>,
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
