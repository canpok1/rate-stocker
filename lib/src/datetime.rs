use chrono::{DateTime, FixedOffset, NaiveDate, Timelike, Utc};

pub fn jst() -> FixedOffset {
    FixedOffset::east_opt(9 * 3600).unwrap()
}

pub fn now_jst() -> DateTime<FixedOffset> {
    let now = Utc::now();
    let jst = jst();
    now.with_timezone(&jst)
}

pub fn today_jst() -> NaiveDate {
    now_jst().date_naive()
}

pub fn to_datetime_jst(date: NaiveDate, hour: u32) -> DateTime<FixedOffset> {
    let datetime = date.and_hms_opt(0, 0, 0).unwrap();
    let datetime_jst = DateTime::from_naive_utc_and_offset(datetime, jst());
    datetime_jst.with_hour(hour).unwrap()
}
