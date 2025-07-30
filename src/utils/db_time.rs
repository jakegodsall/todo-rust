use chrono::{DateTime, Local, NaiveDateTime};



pub const DB_DATETIME_FMT: &str = "%Y-%m-%d %H:%M:%S";

pub fn fmt_local(dt: DateTime<Local>) -> String {
    dt.naive_local().format(DB_DATETIME_FMT).to_string()
}

pub fn parse_local(s: &str) -> DateTime<Local> {
    let naive = NaiveDateTime::parse_from_str(s, DB_DATETIME_FMT)?;
    Local.from_local_datetime(&naive)
        .single();
}