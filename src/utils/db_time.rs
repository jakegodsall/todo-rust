use chrono::{DateTime, Local, MappedLocalTime, NaiveDateTime, ParseError};



pub const DB_DATETIME_FMT: &str = "%Y-%m-%d %H:%M:%S";

pub fn fmt_local(dt: DateTime<Local>) -> String {
    dt.naive_local().format(DB_DATETIME_FMT).to_string()
}

pub fn parse_local(s: &str) -> Result<DateTime<Local>, ParseError> {
    let naive = NaiveDateTime::parse_from_str(s, DB_DATETIME_FMT)?;
    match naive.and_local_timezone(Local) {
        MappedLocalTime::Single(dt) => Ok(dt),
        MappedLocalTime::Ambiguous(_dt0, _dt1) => todo!(),
        MappedLocalTime::None => panic!("invalid date/time"),
    }


}