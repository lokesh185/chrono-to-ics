# Chrono-to-ics

## Description 

this is a app that converts chorono-factorem calendars to `ics` format to be used in various calendar apps like google calendar , Kalendar.

## Done : 
- [x] fetching data from chrono-factorem api 
- [x] parsing the data and linking the data from different api to a single storage struct.
- [x] fetching holidays and days when timetable is changed. 
- [x] writing to the `.ics` file using [ical](https://crates.io/crates/ical) crate. 
- [x] storage of `holidays.json`. 
- [x] gui for the app.
## ToDo : 
- [ ] implementing timetable changes
