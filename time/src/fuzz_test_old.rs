fn run_old(
    year: i32, month: u8, day: u8, hour: u8, 
    minute: u8,
    second: u8, 
    millisecond: u16, 
    microsecond: u32, 
    nanosecond: u32,
    duration_ns: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("year:{:?}, month:{:?}, day:{:?}, hour:{:?}, minute:{:?}, second:{:?}, millisecond:{:?}, micorsecond:{:?}, nanosecond:{:?}", year, month, day, hour, minute, second, millisecond, microsecond, nanosecond);
    // 1. parsing
/*
    // Try converting the byte slice to a string
    if let Ok(input) = std::str::from_utf8(&data) {
        // Fuzz `PrimitiveDateTime::parse` with random strings
        let p1 = PrimitiveDateTime::parse(input, &crate::format_description::well_known::Rfc3339);

        // Fuzz `OffsetDateTime` parsing and formatting
        let p2 = OffsetDateTime::parse(input, &crate::format_description::well_known::Rfc2822);
        println!("p1: {:?}, p2: {:?}", p1, p2);
        // Invalid formats or values should be handled gracefully
    }
 */

    // 2. formatting
    // Try to construct Date, Time, and PrimitiveDateTime
    let date = Date::from_calendar_date(year, Month::try_from(month).unwrap_or(Month::January), day);
    println!("date: {:?}", date);
    
    // 3. basic member functions
    let time = Time::from_hms(hour, minute, second)?;
    let time2 = Time::from_hms_milli(hour, minute, second, millisecond)?;
    let time3 = Time::from_hms_micro(hour, minute, second, microsecond)?;
    let time4 = Time::from_hms_nano(hour, minute, second, nanosecond)?;
    // let time5 = Time::from_hms_nanos_ranged(hour, minute, second, nanosecond);

    for t in [time, time2, time3, time4].iter() {
        assert_eq!(t.hour(), hour);
        assert_eq!(t.minute(), minute);
        assert_eq!(t.second(), second);
        if t == &time2 {
            assert_eq!(t.millisecond(), millisecond);
        }
        if t == &time3 {
            // assert_eq!(t.millisecond(), millisecond);
            assert_eq!(t.microsecond(), microsecond);
        }
        if t == &time4 {
            // assert_eq!(t.millisecond(), millisecond);
            // assert_eq!(t.microsecond(), micorsecond);
            assert_eq!(t.nanosecond(), nanosecond);
        }

        let duration = Duration::nanoseconds(duration_ns);
        let result = *t - duration;

        assert!(result.hour() < 24);  // Wrap-around check for hours
        assert!(result.minute() < 60);  // Wrap-around check for minutes
        assert!(result.second() < 60);  // Wrap-around check for seconds
        assert!(result.millisecond() < 1000);  // Wrap-around check for milliseconds
        assert!(result.microsecond() < 1000000);  // Wrap-around check for microseconds
        assert!(result.nanosecond() < 1000000000);  // Wrap-around check for nanoseconds
        println!("time: {:?} - {:?} = {:?}", t, duration, result);
    }

    // 4. date time offset
    let datetime = PrimitiveDateTime::new(date?, time);
    let datetime2 = PrimitiveDateTime::new(date?, time2);
    let datetime3 = PrimitiveDateTime::new(date?, time3);
    let datetime4 = PrimitiveDateTime::new(date?, time4);
    println!("datetime: {:?}, datetime2: {:?}, datetime3: {:?}, datetime4: {:?}", datetime, datetime2, datetime3, datetime4);
    let offset_datetime = OffsetDateTime::now_utc();
    let formatted = offset_datetime.format(&crate::format_description::well_known::Rfc3339).unwrap();
    if formatted.len() >= 10 {
        println!("Date slice: {}", &formatted[0..10]); // Get YYYY-MM-DD part
    }

    Ok(())
}

#[test]
fn my_fuzz_old() {
    let args: Vec<String> = std::env::args().collect();
    let mut data_arg: Option<String> = None;
    for arg in args.iter().skip(1) {
        if arg.starts_with("data=") {
            data_arg = Some(arg.chars().skip(5).collect());
            break;
        }
    }
    if let Some(data_raw) = data_arg {
        println!("\n- input data: {:?}", data_raw);
        let data: Vec<u8> = data_raw.split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect();

        // Handle potential overflows or invalid values for Date/Time constructors
        let year = i32::from_ne_bytes([data[0], data[1], data[2], data[3]]).clamp(-9999, 9999);
        let month = u8::from_ne_bytes([data[4]]) % 13; // Ensure month is 0-12
        let day = u8::from_ne_bytes([data[5]]) % 32;   // Ensure day is 0-31
        let hour = u8::from_ne_bytes([data[6]]) % 24;  // Ensure hour is 0-23
        let minute = u8::from_ne_bytes([data[7]]) % 60; // Ensure minute is 0-59

        let second = u8::from_ne_bytes([data[8]]) % 60;
        let millisecond = u16::from_ne_bytes([data[9], data[10]]) % 1000;
        let microsecond = u32::from_ne_bytes([data[11], data[12], data[13], data[14]]) % 1000000;
        let nanosecond = u32::from_ne_bytes([data[15], data[16], data[17], data[18]]) % 1000000000;
        // let duration_ns = data[19..].iter().fold(0, |acc, &x| acc * 256 + x as u128);
        let duration_ns = i64::from_ne_bytes([data[19], data[20], data[21], data[22],
            data[23], data[24], data[25], data[26]]); // Ensure minute is 0-59
       // let duration = Duration::from_nanos(duration_ns);

        let res = run_old(year, month, day, hour, minute, second, millisecond, microsecond, nanosecond, duration_ns);
        println!("- result: {:?}", res);
    } else {
        panic!("input data not found");
    }
}

// cargo test --features parsing --features formatting --package time --lib -- fuzz_test::quick_test --exact --show-output
// cargo test --package time --lib -- fuzz_test::quick_test --exact --show-output
#[test]
fn quick_test_old() {
    let data = std::vec![10, 2, 3, 42, 34, 33, 11, 12, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 7, 8, 9, 11, 22, 33, 44, 111, 0, 0, 0, 0, 0, 0, 0];

    let year = i32::from_ne_bytes([data[0], data[1], data[2], data[3]]).clamp(-9999, 9999);
    let month = u8::from_ne_bytes([data[4]]) % 13; // Ensure month is 0-12
    let day = u8::from_ne_bytes([data[5]]) % 32;   // Ensure day is 0-31
    
    let hour = u8::from_ne_bytes([data[6]]) % 24;  // Ensure hour is 0-23
    let minute = u8::from_ne_bytes([data[7]]) % 60; // Ensure minute is 0-59
    let second = u8::from_ne_bytes([data[8]]) % 60;
    let millisecond = u16::from_ne_bytes([data[9], data[10]]) % 1000;
    let microsecond = u32::from_ne_bytes([data[11], data[12], data[13], data[14]]) % 1000000;
    let nanosecond = u32::from_ne_bytes([data[15], data[16], data[17], data[18]]) % 1000000000;
   
    let duration_ns = i64::from_ne_bytes([data[19], data[20], data[21], data[22],
        data[23], data[24], data[25], data[26]]); // Ensure minute is 0-59

    let res = run_old(year, month, day, hour, minute, second, millisecond, microsecond, nanosecond, duration_ns);
    println!("- result: {:?} [{:?}]", res, data.len());
}
