use chrono_to_ics::{api::client::Client, ics};
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
// #[command(author, version, about, long_about = None)]
/// program that converts chrono factorem calendars to .ics files
#[derive(Parser, Default, Debug)]
struct Arguments {
    /// link to the chrono factorem public calendar
    #[arg(short, default_value = "https://www.chrono.crux-bphc.com/view/9Hr7")]
    link: String,
    /// path to the where the `.ics` file shall be saved
    #[arg(short, long, default_value = "content")]
    to_file: PathBuf,
}
#[tokio::main]
async fn main() {
    let m = Arguments::parse();
    dbg!(&m);
    let id = m.link.split('/').last().unwrap().to_string();
    let client = Client::new(id).await.unwrap();
    // dbg!(&client);
    let timetable_str = ics::make_calendar(&client.timetable.unwrap());

    let mut file = File::create("timetable.ics").expect("Unable to create file");
    file.write_all(timetable_str.as_bytes())
        .expect("Unable to write to file");
    // dbg!(test_stuff(&client.timetable.unwrap()));
}
