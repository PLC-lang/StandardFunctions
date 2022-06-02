use chrono::{TimeZone, Timelike};

/// .
/// Converts DT/LDT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DATE_AND_TIME_TO_DATE(input: i64) -> i64 {
    let date_time = chrono::Utc.timestamp_millis(input);

    let new_date_time = date_time.date().and_hms(0, 0, 0);
    new_date_time.timestamp_millis()
}

/// .
/// Converts DT/LDT to TOD/LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DATE_AND_TIME_TO_TIME_OF_DAY(input: i64) -> i64 {
    let date_time = chrono::Utc.timestamp_millis(input);
    let hour = date_time.hour();
    let min = date_time.minute();
    let sec = date_time.second();
    let milli = date_time.timestamp_subsec_millis();

    let new_date_time =
        chrono::NaiveDate::from_ymd(1970, 1, 1).and_hms_milli(hour, min, sec, milli);

    new_date_time.timestamp_millis()
}

/// .
/// Concatenates DATE and TOD to DT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE_TOD(in1: i64, in2: i64) -> i64 {
    let date = chrono::Utc.timestamp_millis(in1).date();
    let tod = chrono::Utc.timestamp_millis(in2);
    let hour = tod.hour();
    let min = tod.minute();
    let sec = tod.second();
    let milli = tod.timestamp_subsec_millis();

    date.and_hms_milli(hour, min, sec, milli).timestamp_millis()
}

/// .
/// Concatenates year, month and day of type INT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__INT(in1: i16, in2: i16, in3: i16) -> i64 {
    let date = chrono::NaiveDate::from_ymd(in1.into(), in2 as u32, in3 as u32);
    let dt = date.and_hms(0, 0, 0);
    dt.timestamp_millis()
}

/// .
/// Concatenates year, month and day of type UINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__UINT(in1: u16, in2: u16, in3: u16) -> i64 {
    let date = chrono::NaiveDate::from_ymd(in1.into(), in2.into(), in3.into());
    let dt = date.and_hms(0, 0, 0);
    dt.timestamp_millis()
}

/// .
/// Concatenates year, month and day of type DINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__DINT(in1: i32, in2: i32, in3: i32) -> i64 {
    let date = chrono::NaiveDate::from_ymd(in1, in2 as u32, in3 as u32);
    let dt = date.and_hms(0, 0, 0);
    dt.timestamp_millis()
}

/// .
/// Concatenates year, month and day of type UDINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__UDINT(in1: u32, in2: u32, in3: u32) -> i64 {
    let date = chrono::NaiveDate::from_ymd(in1 as i32, in2, in3);
    let dt = date.and_hms(0, 0, 0);
    dt.timestamp_millis()
}

/// .
/// Concatenates year, month and day of type LINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__LINT(in1: i64, in2: i64, in3: i64) -> i64 {
    let date = chrono::NaiveDate::from_ymd(in1 as i32, in2 as u32, in3 as u32);
    let dt = date.and_hms(0, 0, 0);
    dt.timestamp_millis()
}

/// .
/// Concatenates year, month and day of type ULINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__ULINT(in1: u64, in2: u64, in3: u64) -> i64 {
    let date = chrono::NaiveDate::from_ymd(in1 as i32, in2 as u32, in3 as u32);
    let dt = date.and_hms(0, 0, 0);
    dt.timestamp_millis()
}

/// .
/// Concatenates hour, minute, second, millisecond of type SINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__SINT(in1: i8, in2: i8, in3: i8, in4: i8) -> i64 {
    let date = chrono::NaiveDate::from_ymd(1970, 1, 1);
    let dt = date.and_hms_milli(in1 as u32, in2 as u32, in3 as u32, in4 as u32);
    dt.timestamp_millis()
}

/// .
/// Concatenates hour, minute, second, millisecond of type USINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__USINT(in1: u8, in2: u8, in3: u8, in4: u8) -> i64 {
    let date = chrono::NaiveDate::from_ymd(1970, 1, 1);
    let dt = date.and_hms_milli(in1 as u32, in2 as u32, in3 as u32, in4 as u32);
    dt.timestamp_millis()
}

/// .
/// Concatenates hour, minute, second, millisecond of type INT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__INT(in1: i16, in2: i16, in3: i16, in4: i16) -> i64 {
    let date = chrono::NaiveDate::from_ymd(1970, 1, 1);
    let dt = date.and_hms_milli(in1 as u32, in2 as u32, in3 as u32, in4 as u32);
    dt.timestamp_millis()
}

/// .
/// Concatenates hour, minute, second, millisecond of type UINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__UINT(in1: u16, in2: u16, in3: u16, in4: u16) -> i64 {
    let date = chrono::NaiveDate::from_ymd(1970, 1, 1);
    let dt = date.and_hms_milli(in1 as u32, in2 as u32, in3 as u32, in4 as u32);
    dt.timestamp_millis()
}

/// .
/// Concatenates hour, minute, second, millisecond of type DINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__DINT(in1: i32, in2: i32, in3: i32, in4: i32) -> i64 {
    let date = chrono::NaiveDate::from_ymd(1970, 1, 1);
    let dt = date.and_hms_milli(in1 as u32, in2 as u32, in3 as u32, in4 as u32);
    dt.timestamp_millis()
}

/// .
/// Concatenates hour, minute, second, millisecond of type UDINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__UDINT(in1: u32, in2: u32, in3: u32, in4: u32) -> i64 {
    let date = chrono::NaiveDate::from_ymd(1970, 1, 1);
    let dt = date.and_hms_milli(in1, in2 as u32, in3, in4);
    dt.timestamp_millis()
}

/// .
/// Concatenates hour, minute, second, millisecond of type LINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__LINT(in1: i64, in2: i64, in3: i64, in4: i64) -> i64 {
    let date = chrono::NaiveDate::from_ymd(1970, 1, 1);
    let dt = date.and_hms_milli(in1 as u32, in2 as u32, in3 as u32, in4 as u32);
    dt.timestamp_millis()
}

/// .
/// Concatenates hour, minute, second, millisecond of type ULINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__ULINT(in1: u64, in2: u64, in3: u64, in4: u64) -> i64 {
    let date = chrono::NaiveDate::from_ymd(1970, 1, 1);
    let dt = date.and_hms_milli(in1 as u32, in2 as u32, in3 as u32, in4 as u32);
    dt.timestamp_millis()
}
