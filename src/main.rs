use chrono_to_ics::api;
use clap::Parser;
use std::{fs, path::PathBuf};

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
    let data = api::Data::new(id).await.unwrap();
    dbg!(&data);
    let datetime = iso8601::datetime("2023-05-17T08:30:00.000Z").unwrap();
    dbg!(iso8601::datetime("2023-05-17T08:30:00.000Z").unwrap());
}
