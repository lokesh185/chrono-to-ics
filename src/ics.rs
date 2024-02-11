use crate::api::data::{Section, TimeTable, Timing};
use chrono::{DateTime, Datelike, Days, Duration, Timelike, Utc, Weekday};
use icalendar::{Calendar, Component, Event, EventLike, Property};
const UTC_DATE_TIME_FORMAT: &str = "%Y%m%dT%H%M%SZ";
#[derive(Debug)]
struct EventGen {
    summary: String,
    description: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    weekdays: Vec<Weekday>,
    recurence_end: DateTime<Utc>,
    exdates: Vec<DateTime<Utc>>,
    location: String,
}

impl EventGen {
    fn new(
        summary: String,
        description: String,
        section: &Section,
        sem_start: &DateTime<Utc>,
        sem_end: &DateTime<Utc>,
        holidays: &[DateTime<Utc>],
    ) -> Vec<Self> {
        let mut timing_sets: Vec<Vec<Timing>> = vec![vec![]];
        // make vec<timing> into sets where the start and end are the same
        section.timings.iter().for_each(|timing| {
            let mut found = false;
            'outer: for inner_timing_set in timing_sets.iter_mut() {
                for inner_timing in inner_timing_set.iter() {
                    if inner_timing.start == timing.start && inner_timing.end == timing.end {
                        inner_timing_set.push(timing.clone());
                        found = true;
                        break 'outer;
                    }
                }
            }
            if !found {
                timing_sets.push(vec![timing.clone()])
            };
        });
        // let exdates = holidays.iter().map(|holiday| holiday.date.with_hour(1+timing_vec.first()?.start))

        dbg!(&timing_sets);
        timing_sets
            .iter()
            .filter_map(|timing_vec| {
                let weekday_vec = timing_vec
                    .iter()
                    .map(|timing| timing.day)
                    .collect::<Vec<Weekday>>();
                Some(EventGen {
                    summary: summary.clone(),
                    description: description.clone(),
                    start_time: start_time(sem_start, &weekday_vec, timing_vec.first()?.start),
                    end_time: end_time(sem_start, &weekday_vec, timing_vec.first()?.end),
                    weekdays: weekday_vec,
                    recurence_end: sem_end.clone(),
                    exdates: holidays
                        .iter()
                        .map(|date_time| {
                            date_time
                                .with_hour(1 + timing_vec.first().unwrap().start as u32)
                                .unwrap()
                                .checked_add_days(Days::new(1))
                                .unwrap()
                        })
                        .collect::<Vec<DateTime<Utc>>>(),
                    location: timing_vec.first().unwrap().classroom.clone(),
                })
            })
            .collect::<Vec<Self>>()
    }
    fn to_event(&self) -> Event {
        let exd = self
            .exdates
            .iter()
            .map(|datetime| datetime.format(UTC_DATE_TIME_FORMAT).to_string())
            .collect::<Vec<String>>()
            .join(",");
        //EXDATE must have same time as event start
        Event::new()
            .summary(&self.summary)
            .description(&self.description)
            .add_property(
                "DTSTART",
                self.start_time
                    .format(UTC_DATE_TIME_FORMAT)
                    .to_string()
                    .as_str(),
            )
            .add_property(
                "RRULE",
                format!(
                    "FREQ=WEEKLY;UNTIL={};BYDAY={}",
                    self.recurence_end.format(UTC_DATE_TIME_FORMAT).to_string(),
                    weekdays_to_string(&self.weekdays)
                )
                .as_str(),
            )
            .add_property("EXDATE", &exd)
            .add_property(
                "DTEND",
                self.end_time
                    .format(UTC_DATE_TIME_FORMAT)
                    .to_string()
                    .as_str(),
            )
            .add_property("LOCATION", self.location.as_str())
            .add_property("TRANSP", "TRANSPARENT")
            .done()
    }
}
fn weekdays_to_string(weekdays: &Vec<Weekday>) -> String {
    weekdays
        .iter()
        .map(|weekday| {
            match weekday {
                Weekday::Mon => "MO",
                Weekday::Tue => "TU",
                Weekday::Wed => "WE",
                Weekday::Thu => "TH",
                Weekday::Fri => "FR",
                Weekday::Sat => "SA",
                Weekday::Sun => "SU",
            }
            .to_string()
        })
        .collect::<Vec<String>>()
        .join(",")
}

fn start_time(
    sem_start: &DateTime<Utc>,
    weekday: &Vec<Weekday>,
    timing_start: u8,
) -> DateTime<Utc> {
    let mut date = sem_start.clone();
    while !weekday.contains(&date.weekday()) {
        date = date + Duration::days(1);
    }
    // set hours from 1 -> 8am ,2 ->9 am
    //set hour to 2:30 am
    date = date.with_hour(1 + timing_start as u32).unwrap();
    date = date.with_minute(30).unwrap();
    date
}
fn end_time(sem_start: &DateTime<Utc>, weekday: &Vec<Weekday>, timing_start: u8) -> DateTime<Utc> {
    let mut date = sem_start.clone();
    while !weekday.contains(&date.weekday()) {
        date = date + Duration::days(1);
    }
    // set hours from 1 -> 3:20am ,2 ->4:20 am
    //set hour to 3:30 am
    date = date.with_hour(2 + timing_start as u32).unwrap();
    date = date.with_minute(20).unwrap();
    date
}

fn generate_exam_event(
    exam_start: &DateTime<Utc>,
    exam_end: &DateTime<Utc>,
    summary: &str,
) -> Event {
    Event::new()
        .summary(summary)
        .starts(*exam_start)
        .ends(*exam_end)
        .description("something ")
        .done()
}
pub fn make_calendar(time_table: &TimeTable) -> String {
    let mut calendar = Calendar::new();
    calendar.append_property(Property::new("NAME", "bphc calendar"));

    let mut holidays = time_table
        .holidays
        .iter()
        .map(|hoilday| hoilday.date)
        .collect::<Vec<DateTime<Utc>>>();

    if let Some((mut mid_sem_start, mid_sem_end)) = time_table.midsem_dates {
        while mid_sem_start <= mid_sem_end {
            holidays.push(mid_sem_start.clone());
            mid_sem_start = mid_sem_start.checked_add_days(Days::new(1)).unwrap();
        }
    }
    let mut events: Vec<EventGen> = vec![];
    for course in &time_table.courses {
        if let Some(section) = &course.lecture {
            events.extend(EventGen::new(
                format!("Lec:{} ", course.name),
                course.code.clone(),
                section,
                &time_table.classwork_start,
                &time_table.classwork_end,
                &holidays,
            ));
        }
        if let Some(section) = &course.lab {
            events.extend(EventGen::new(
                format!("Lab: {} ", course.name),
                course.code.clone(),
                section,
                &time_table.classwork_start,
                &time_table.classwork_end,
                &holidays,
            ));
        }
        if let Some(section) = &course.tutorial {
            events.extend(EventGen::new(
                format!("Tut:{} ", course.name),
                course.code.clone(),
                section,
                &time_table.classwork_start,
                &time_table.classwork_end,
                &holidays,
            ));
        }
        if let Some((midsem_start, midsem_end)) = &course.midsem_date_time {
            calendar.push(generate_exam_event(
                midsem_start,
                midsem_end,
                &format!("Exam: {}", course.name),
            ));
        }
        if let Some((compre_start, compre_end)) = &course.compre_date_time {
            calendar.push(generate_exam_event(
                compre_start,
                compre_end,
                &format!("Exam: {}", course.name),
            ));
        }
    }

    for eventgen in events {
        calendar.push(eventgen.to_event());
    }
    calendar.to_string()
}
