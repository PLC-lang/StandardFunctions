use chrono::{TimeZone, Timelike};

/// .
/// Converts DT/LDT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DATE_AND_TIME_TO_DATE(input: i64) -> i64 {
    let date_time = chrono::Utc.timestamp_nanos(input);

    let new_date_time = date_time.date().and_hms(0, 0, 0);
    new_date_time.timestamp_nanos()
}

/// .
/// Converts DT/LDT to TOD/LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DATE_AND_TIME_TO_TIME_OF_DAY(input: i64) -> i64 {
    let date_time = chrono::Utc.timestamp_nanos(input);
    let hour = date_time.hour();
    let min = date_time.minute();
    let sec = date_time.second();
    let nano = date_time.timestamp_subsec_nanos();

    let new_date_time = chrono::NaiveDate::from_ymd(1970, 1, 1).and_hms_nano(hour, min, sec, nano);

    new_date_time.timestamp_nanos()
}
