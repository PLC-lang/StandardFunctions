use chrono::{Datelike, TimeZone, Timelike};

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
    concat_date(in1.into(), in2 as u32, in3 as u32)
}

/// .
/// Concatenates year, month and day of type UINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__UINT(in1: u16, in2: u16, in3: u16) -> i64 {
    concat_date(in1.into(), in2.into(), in3.into())
}

/// .
/// Concatenates year, month and day of type DINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__DINT(in1: i32, in2: i32, in3: i32) -> i64 {
    concat_date(in1, in2 as u32, in3 as u32)
}

/// .
/// Concatenates year, month and day of type UDINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__UDINT(in1: u32, in2: u32, in3: u32) -> i64 {
    concat_date(in1 as i32, in2, in3)
}

/// .
/// Concatenates year, month and day of type LINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__LINT(in1: i64, in2: i64, in3: i64) -> i64 {
    concat_date(in1 as i32, in2 as u32, in3 as u32)
}

/// .
/// Concatenates year, month and day of type ULINT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_DATE__ULINT(in1: u64, in2: u64, in3: u64) -> i64 {
    concat_date(in1 as i32, in2 as u32, in3 as u32)
}

/// .
/// Concatenates year, month and day to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn concat_date(in1: i32, in2: u32, in3: u32) -> i64 {
    let date = chrono::NaiveDate::from_ymd(in1, in2, in3);
    let dt = date.and_hms(0, 0, 0);
    dt.timestamp_millis()
}

/// .
/// Concatenates hour, minute, second, millisecond of type SINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__SINT(in1: i8, in2: i8, in3: i8, in4: i8) -> i64 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type USINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__USINT(in1: u8, in2: u8, in3: u8, in4: u8) -> i64 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type INT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__INT(in1: i16, in2: i16, in3: i16, in4: i16) -> i64 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type UINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__UINT(in1: u16, in2: u16, in3: u16, in4: u16) -> i64 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type DINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__DINT(in1: i32, in2: i32, in3: i32, in4: i32) -> i64 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type UDINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__UDINT(in1: u32, in2: u32, in3: u32, in4: u32) -> i64 {
    concat_tod(in1, in2 as u32, in3, in4)
}

/// .
/// Concatenates hour, minute, second, millisecond of type LINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__LINT(in1: i64, in2: i64, in3: i64, in4: i64) -> i64 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond of type ULINT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CONCAT_TOD__ULINT(in1: u64, in2: u64, in3: u64, in4: u64) -> i64 {
    concat_tod(in1 as u32, in2 as u32, in3 as u32, in4 as u32)
}

/// .
/// Concatenates hour, minute, second, millisecond to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn concat_tod(in1: u32, in2: u32, in3: u32, in4: u32) -> i64 {
    let date = chrono::NaiveDate::from_ymd(1970, 1, 1);
    let dt = date.and_hms_milli(in1, in2, in3, in4);
    dt.timestamp_millis()
}

/// .
/// Splits DATE into year, month, day of type INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DATE__INT(in1: i64, out1: &mut i16, out2: &mut i16, out3: &mut i16) -> i16 {
    let date = chrono::Utc.timestamp_millis(in1).date();
    // if year does not fit in target data type -> panic
    *out1 = date.year().try_into().unwrap();
    *out2 = date.month() as i16;
    *out3 = date.day() as i16;

    0
}

/// .
/// Splits DATE into year, month, day of type UINT
/// Panics on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DATE__UINT(
    in1: i64,
    out1: &mut u16,
    out2: &mut u16,
    out3: &mut u16,
) -> i16 {
    let date = chrono::Utc.timestamp_millis(in1).date();
    // if year does not fit in target data type -> panic
    *out1 = date.year().try_into().unwrap();
    *out2 = date.month() as u16;
    *out3 = date.day() as u16;

    0
}

/// .
/// Splits DATE into year, month, day of type DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DATE__DINT(
    in1: i64,
    out1: &mut i32,
    out2: &mut i32,
    out3: &mut i32,
) -> i16 {
    let date = chrono::Utc.timestamp_millis(in1).date();
    *out1 = date.year();
    *out2 = date.month() as i32;
    *out3 = date.day() as i32;

    0
}

/// .
/// Splits DATE into year, month, day of type UDINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DATE__UDINT(
    in1: i64,
    out1: &mut u32,
    out2: &mut u32,
    out3: &mut u32,
) -> i16 {
    let date = chrono::Utc.timestamp_millis(in1).date();
    // if year does not fit in target data type -> panic
    *out1 = date.year().try_into().unwrap();
    *out2 = date.month() as u32;
    *out3 = date.day() as u32;

    0
}

/// .
/// Splits DATE into year, month, day of type LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DATE__LINT(
    in1: i64,
    out1: &mut i64,
    out2: &mut i64,
    out3: &mut i64,
) -> i16 {
    let date = chrono::Utc.timestamp_millis(in1).date();
    // if year does not fit in target data type -> panic
    *out1 = date.year().try_into().unwrap();
    *out2 = date.month() as i64;
    *out3 = date.day() as i64;

    0
}

/// .
/// Splits DATE into year, month, day of type ULINT
/// Panics on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DATE__ULINT(
    in1: i64,
    out1: &mut u64,
    out2: &mut u64,
    out3: &mut u64,
) -> i16 {
    let date = chrono::Utc.timestamp_millis(in1).date();
    // if year does not fit in target data type -> panic
    *out1 = date.year().try_into().unwrap();
    *out2 = date.month() as u64;
    *out3 = date.day() as u64;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__INT(
    in1: i64,
    out1: &mut i16,
    out2: &mut i16,
    out3: &mut i16,
    out4: &mut i16,
) -> i16 {
    let tod = chrono::Utc.timestamp_millis(in1);
    *out1 = tod.hour() as i16;
    *out2 = tod.minute() as i16;
    *out3 = tod.second() as i16;
    *out4 = tod.timestamp_subsec_millis() as i16;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type UINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__UINT(
    in1: i64,
    out1: &mut u16,
    out2: &mut u16,
    out3: &mut u16,
    out4: &mut u16,
) -> i16 {
    let tod = chrono::Utc.timestamp_millis(in1);
    *out1 = tod.hour() as u16;
    *out2 = tod.minute() as u16;
    *out3 = tod.second() as u16;
    *out4 = tod.timestamp_subsec_millis() as u16;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__DINT(
    in1: i64,
    out1: &mut i32,
    out2: &mut i32,
    out3: &mut i32,
    out4: &mut i32,
) -> i16 {
    let tod = chrono::Utc.timestamp_millis(in1);
    *out1 = tod.hour() as i32;
    *out2 = tod.minute() as i32;
    *out3 = tod.second() as i32;
    *out4 = tod.timestamp_subsec_millis() as i32;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type UDINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__UDINT(
    in1: i64,
    out1: &mut u32,
    out2: &mut u32,
    out3: &mut u32,
    out4: &mut u32,
) -> i16 {
    let tod = chrono::Utc.timestamp_millis(in1);
    *out1 = tod.hour() as u32;
    *out2 = tod.minute() as u32;
    *out3 = tod.second() as u32;
    *out4 = tod.timestamp_subsec_millis() as u32;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__LINT(
    in1: i64,
    out1: &mut i64,
    out2: &mut i64,
    out3: &mut i64,
    out4: &mut i64,
) -> i16 {
    let tod = chrono::Utc.timestamp_millis(in1);
    *out1 = tod.hour() as i64;
    *out2 = tod.minute() as i64;
    *out3 = tod.second() as i64;
    *out4 = tod.timestamp_subsec_millis() as i64;

    0
}

/// .
/// Splits TOD into hour, minute, second, millisecond of type ULINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_TOD__ULINT(
    in1: i64,
    out1: &mut u64,
    out2: &mut u64,
    out3: &mut u64,
    out4: &mut u64,
) -> i16 {
    let tod = chrono::Utc.timestamp_millis(in1);
    *out1 = tod.hour() as u64;
    *out2 = tod.minute() as u64;
    *out3 = tod.second() as u64;
    *out4 = tod.timestamp_subsec_millis() as u64;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__INT(
    in1: i64,
    out1: &mut i16,
    out2: &mut i16,
    out3: &mut i16,
    out4: &mut i16,
    out5: &mut i16,
    out6: &mut i16,
    out7: &mut i16,
) -> i16 {
    let dt = chrono::Utc.timestamp_millis(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().try_into().unwrap();
    *out2 = dt.month() as i16;
    *out3 = dt.day() as i16;
    *out4 = dt.hour() as i16;
    *out5 = dt.minute() as i16;
    *out6 = dt.second() as i16;
    *out7 = dt.timestamp_subsec_millis() as i16;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type UINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__UINT(
    in1: i64,
    out1: &mut u16,
    out2: &mut u16,
    out3: &mut u16,
    out4: &mut u16,
    out5: &mut u16,
    out6: &mut u16,
    out7: &mut u16,
) -> i16 {
    let dt = chrono::Utc.timestamp_millis(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().try_into().unwrap();
    *out2 = dt.month() as u16;
    *out3 = dt.day() as u16;
    *out4 = dt.hour() as u16;
    *out5 = dt.minute() as u16;
    *out6 = dt.second() as u16;
    *out7 = dt.timestamp_subsec_millis() as u16;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__DINT(
    in1: i64,
    out1: &mut i32,
    out2: &mut i32,
    out3: &mut i32,
    out4: &mut i32,
    out5: &mut i32,
    out6: &mut i32,
    out7: &mut i32,
) -> i16 {
    let dt = chrono::Utc.timestamp_millis(in1);
    *out1 = dt.year();
    *out2 = dt.month() as i32;
    *out3 = dt.day() as i32;
    *out4 = dt.hour() as i32;
    *out5 = dt.minute() as i32;
    *out6 = dt.second() as i32;
    *out7 = dt.timestamp_subsec_millis() as i32;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type UDINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__UDINT(
    in1: i64,
    out1: &mut u32,
    out2: &mut u32,
    out3: &mut u32,
    out4: &mut u32,
    out5: &mut u32,
    out6: &mut u32,
    out7: &mut u32,
) -> i16 {
    let dt = chrono::Utc.timestamp_millis(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().try_into().unwrap();
    *out2 = dt.month() as u32;
    *out3 = dt.day() as u32;
    *out4 = dt.hour() as u32;
    *out5 = dt.minute() as u32;
    *out6 = dt.second() as u32;
    *out7 = dt.timestamp_subsec_millis() as u32;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__LINT(
    in1: i64,
    out1: &mut i64,
    out2: &mut i64,
    out3: &mut i64,
    out4: &mut i64,
    out5: &mut i64,
    out6: &mut i64,
    out7: &mut i64,
) -> i16 {
    let dt = chrono::Utc.timestamp_millis(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().try_into().unwrap();
    *out2 = dt.month() as i64;
    *out3 = dt.day() as i64;
    *out4 = dt.hour() as i64;
    *out5 = dt.minute() as i64;
    *out6 = dt.second() as i64;
    *out7 = dt.timestamp_subsec_millis() as i64;

    0
}

/// .
/// Splits DT into year, month, day, hour, minute, second, millisecond of type ULINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SPLIT_DT__ULINT(
    in1: i64,
    out1: &mut u64,
    out2: &mut u64,
    out3: &mut u64,
    out4: &mut u64,
    out5: &mut u64,
    out6: &mut u64,
    out7: &mut u64,
) -> i16 {
    let dt = chrono::Utc.timestamp_millis(in1);
    // if year does not fit in target data type -> panic
    *out1 = dt.year().try_into().unwrap();
    *out2 = dt.month() as u64;
    *out3 = dt.day() as u64;
    *out4 = dt.hour() as u64;
    *out5 = dt.minute() as u64;
    *out6 = dt.second() as u64;
    *out7 = dt.timestamp_subsec_millis() as u64;

    0
}

/// .
/// Returns day of week for given DATE of type SINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DAY_OF_WEEK__SINT(in1: i64, out1: &mut i8) -> i8 {
    let date = chrono::Utc.timestamp_millis(in1);
    let day = date.weekday().num_days_from_sunday() as i8;
    *out1 = day;
    day
}

/// .
/// Returns day of week for given DATE of type USINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DAY_OF_WEEK__USINT(in1: i64, out1: &mut u8) -> u8 {
    day_of_week(in1, out1)
}

/// .
/// Returns day of week for given DATE of type INT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DAY_OF_WEEK__INT(in1: i64, out1: &mut i16) -> i16 {
    day_of_week(in1, out1)
}

/// .
/// Returns day of week for given DATE of type UINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DAY_OF_WEEK__UINT(in1: i64, out1: &mut u16) -> u16 {
    day_of_week(in1, out1)
}

/// .
/// Returns day of week for given DATE of type DINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DAY_OF_WEEK__DINT(in1: i64, out1: &mut i32) -> i32 {
    day_of_week(in1, out1)
}

/// .
/// Returns day of week for given DATE of type UDINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DAY_OF_WEEK__UDINT(in1: i64, out1: &mut u32) -> u32 {
    day_of_week(in1, out1)
}

/// .
/// Returns day of week for given DATE of type LINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DAY_OF_WEEK__LINT(in1: i64, out1: &mut i64) -> i64 {
    day_of_week(in1, out1)
}

/// .
/// Returns day of week for given DATE of type ULINT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DAY_OF_WEEK__ULINT(in1: i64, out1: &mut u64) -> u64 {
    day_of_week(in1, out1)
}

fn day_of_week<T: From<u8> + Copy>(in1: i64, out1: &mut T) -> T {
    let date = chrono::Utc.timestamp_millis(in1);
    // num_days_from_sunday() will always be able to convert to u8, range 0-6
    let day = T::from(date.weekday().num_days_from_sunday() as u8);
    *out1 = day;
    day
}