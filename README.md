# Chrono-to-ics

## Description 

this is a app that converts chorono-factorem calendars to `ics` format to be used in various calendar apps like google calendar , Kalendar.

## Done : 
- [x] fetching data from chrono-factorem api 
- [x] parsing the data and linking the data from different api to a single storage struct.
- [x] fetching holidays and days when timetable is changed. 
## ToDo : 

- [ ] writing to the `.ics` file using [ical](https://crates.io/crates/ical) crate. 
- [ ] storage of `holidays.json`. 
- [ ] gui for the app.
  
### GUI 
it just needs a one text box for the chorno link , a folder dialog box and a button . 