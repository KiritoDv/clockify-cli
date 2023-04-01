use chrono::{DateTime, Local, NaiveDateTime, NaiveTime, Offset, TimeZone, Utc, NaiveDate};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, Write},
};

lazy_static! {
    static ref TIME_NAMES: HashMap<&'static str, &'static str> =
        vec![("H", "hours"), ("M", "minutes"), ("S", "seconds")]
            .into_iter()
            .collect();
}

pub(crate) fn parse_duration(duration: &str) -> String {
    let mut output = String::new();
    let regex = Regex::new(r"[a-zA-Z]").unwrap();
    let result = regex.split(duration);
    let process = result.collect::<Vec<&str>>();
    let matches = duration
        .match_indices(&regex)
        .collect::<Vec<(usize, &str)>>();
    match process.len() - 1 {
        1 => {
            let time = process[0].parse::<u32>().unwrap();

            if time == 0 {
                return String::from("No time registered");
            }

            output.push_str(&format!(
                "{} {}",
                time,
                TIME_NAMES.get(&matches[0].1).unwrap()
            ));
        }
        2 => output.push_str(&format!(
            "{} {} & {} {}",
            process[0],
            TIME_NAMES.get(&matches[0].1).unwrap(),
            process[1],
            TIME_NAMES.get(&matches[1].1).unwrap()
        )),
        3 => output.push_str(&format!(
            "{} {}, {} {} & {} {}",
            process[0],
            TIME_NAMES.get(&matches[0].1).unwrap(),
            process[1],
            TIME_NAMES.get(&matches[1].1).unwrap(),
            process[2],
            TIME_NAMES.get(&matches[2].1).unwrap()
        )),
        _ => output.push_str("No time registered"),
    }
    output
}

pub(crate) fn clear_screen() {
    let mut stdout = io::stdout();
    write!(stdout, "{esc}c", esc = 27 as char).unwrap();
    stdout.flush().unwrap()
}

pub(crate) fn cursor() {
    let mut stdout = io::stdout();
    write!(stdout, "> ").unwrap();
    stdout.flush().unwrap()
}

pub(crate) fn read<T>() -> Option<T>
where
    T: std::str::FromStr,
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>().ok()
}

pub(crate) fn date(time: NaiveTime) -> DateTime<Utc> {
    let now = Utc::now();
    let local = Local.timestamp_opt(0, 0).unwrap().offset().fix();
    let date = NaiveDateTime::new(now.date_naive(), time);
    let date = local.from_local_datetime(&date).unwrap();
    date.with_timezone(&Utc)
}

pub(crate) fn datetime(date: NaiveDate, time: NaiveTime) -> DateTime<Utc> {
    let local = Local.timestamp_opt(0, 0).unwrap().offset().fix();
    let date = NaiveDateTime::new(date, time);
    let date = local.from_local_datetime(&date).unwrap();
    date.with_timezone(&Utc)
}
