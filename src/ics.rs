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
} // he;p
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
        // make vec<timing> into sets where the both start and end are the same
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
                    start_time: start_time(sem_start, &weekday_vec, timing_vec.first()?.start)?,
                    end_time: end_time(sem_start, &weekday_vec, timing_vec.first()?.end)?,
                    weekdays: weekday_vec,
                    recurence_end: sem_end.clone(),
                    exdates: holidays
                        .iter()
                        .filter_map(|date_time| {
                            Some(
                                date_time
                                    .with_hour(1 + timing_vec.first()?.start as u32)?
                                    .checked_add_days(Days::new(1))?,
                            )
                        })
                        .collect::<Vec<DateTime<Utc>>>(),
                    location: timing_vec.first()?.classroom.clone(),
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
) -> Option<DateTime<Utc>> {
    let mut date = sem_start.clone();
    while !weekday.contains(&date.weekday()) {
        date = date + Duration::days(1);
    }
    // set hours from 1 -> 2:30am ,2 ->3:30 am
    // 2:30 am UTC = 8 am IST
    Some(date.with_hour(1 + timing_start as u32)?.with_minute(30)?)
}
fn end_time(
    sem_start: &DateTime<Utc>,
    weekday: &Vec<Weekday>,
    timing_start: u8,
) -> Option<DateTime<Utc>> {
    let mut date = sem_start.clone();
    while !weekday.contains(&date.weekday()) {
        date = date + Duration::days(1);
    }
    // set hours from 1 -> 3:20am ,2 ->4:20 am
    // 3:20 am UTC = 8:50 am IST

    Some(date.with_hour(2 + timing_start as u32)?.with_minute(20)?)
}

fn generate_exam_event(
    exam_start: &DateTime<Utc>,
    exam_end: &DateTime<Utc>,
    summary: &str,
) -> Event {
    // uses the current because crux didnt update years in their exam datetimes .
    // maybe will not work if the semester is across 2 calendar years.
    let cur_year = Utc::now().year();
    Event::new()
        .summary(summary)
        .starts(
            exam_start
                .with_year(cur_year)
                .unwrap_or_else(|| *exam_start),
        )
        .ends(exam_end.with_year(cur_year).unwrap_or_else(|| *exam_end))
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
            mid_sem_start = match mid_sem_start.checked_add_days(Days::new(1)) {
                Some(new_mid_sem_date) => new_mid_sem_date,
                None => {
                    break;
                }
            };
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
