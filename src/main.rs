use chrono_to_ics::{api::client::ApiClient, ics};
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
// #[command(author, version, about, long_about = None)]
/// program that converts chrono factorem calendars to .ics files
#[derive(Parser, Default, Debug)]
struct Arguments {
    /// link to the chrono factorem public calendar
    #[arg(short)]
    link: String,
    /// path to the where the `.ics` file shall be saved
    #[arg(short, long, default_value = "content")]
    to_file: PathBuf,
}
fn main() {
    let m = Arguments::parse();
    let mut client = ApiClient::new(get_id_from_link(&m.link).unwrap()).unwrap();
    client.fetch_timetable().unwrap();
    client.update_time_table().unwrap();
    let timetable_string = ics::make_calendar(&client.timetable.unwrap());

    write_to_file(&timetable_string).unwrap();
}

fn get_id_from_link(link: &str) -> Option<String> {
    Some(link.split('/').last()?.to_string())
}

fn write_to_file(data: &String) -> Option<()> {
    let mut file = File::create("timetable.ics").unwrap();
    file.write_all(data.as_bytes()).unwrap();
    Some(())
}
