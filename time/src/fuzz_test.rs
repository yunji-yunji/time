#![cfg(feature = "parsing")]
use std::vec;
use std::vec::*;
use std::println;
use std::string::String;
use crate::Duration;
use crate::{Date, Time, UtcOffset, Month};

use crate::ext::NumericalDuration;
// use crate::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time};
// use crate::ext::numerical_duration::NumericalDuration;

// cargo test --test tests quick_test -- --show-output
#[test]
fn quick_test() {
    let data = vec![
        10, 2, 3, 42, 34, 33, 11, 12, 111, 24, 56, 78, 98, 23, 45, 67, 89, 100,
        9, 5, 16, 28, 37, 50, 73, 61, 82, 93, 1, 4, 6, 7, 8, 14, 15, 17, 18, 19,
        20, 21, 22, 25, 26, 27, 29, 30, 31, 32, 35, 36, 38, 39, 40, 41, 43, 44,
        46, 47, 48, 49, 51, 52, 53, 54, 55, 57, 58, 59, 60, 62, 63, 64, 65, 66,
        68, 69, 70, 71, 72, 74, 75, 76, 77, 79, 80, 81, 83, 84, 85, 86, 87, 88,
        90, 91, 92, 94, 95, 96, 97, 99, 101, 102, 103, 104, 105, 106, 107, 108,
        109, 110, 112, 112,
    ];

    // handle potential overflows or invalid values for Date/Time constructors
    let weeks = i64::from_ne_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]) / 604_800;
    let days  = i64::from_ne_bytes([data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]]) / 86_400;
    let hours = i64::from_ne_bytes([data[16], data[17], data[18], data[19], data[20], data[21], data[22], data[23]]) / 3_600;
    let minutes = i64::from_ne_bytes([data[24], data[25], data[26], data[27], data[28], data[29], data[30], data[31]]) / 60;
    let seconds = i64::from_ne_bytes([data[32], data[33], data[34], data[35], data[36], data[37], data[38], data[39]]);
    let milliseconds = i64::from_ne_bytes([data[40], data[41], data[42], data[43], data[44], data[45], data[46], data[47]]);
    let microseconds = i64::from_ne_bytes([data[48], data[49], data[50], data[51], data[52], data[53], data[54], data[55]]);
    let nanoseconds = i64::from_ne_bytes([data[56], data[57], data[58], data[59], data[60], data[61], data[62], data[63]]);
    let nanoseconds_i128 = i128::from_ne_bytes([data[64], data[65], data[66], data[67], data[68], data[69], data[70], data[71], data[72], data[73], data[74], data[75],data[76], data[77], data[78], data[79]]);
    let nanoseconds_i32 = i32::from_ne_bytes([data[80], data[81], data[82], data[83]]);
    let f64_num = f64::from_ne_bytes([data[84], data[85], data[86], data[87], data[88], data[89], data[90], data[91]]);
    let f32_num = f32::from_ne_bytes([data[92], data[93], data[94], data[95]]);
    let numerical1 = f64::from_ne_bytes([data[96], data[97], data[98], data[99], data[100], data[101], data[102], data[103]]);
    let numerical2 = i64::from_ne_bytes([data[104], data[105], data[106], data[107], data[108], data[109], data[110], data[111]]);
    println!("weeks: {:?}\ndays: {:?}\nhours: {:?}\nminutes: {:?}\nseconds: {:?}\nmilliseconds: {:?}\nmicroseconds: {:?}\nnanoseconds: {:?}\nnanoseconds_i128: {:?}\nnanoseconds_i32: {:?}\nf64_num: {:?}\nf32_num: {:?}\nnumerical1: {:?}\nnumerical2: {:?}", weeks, days, hours, minutes, seconds, milliseconds, microseconds, nanoseconds, nanoseconds_i128, nanoseconds_i32, f64_num, f32_num, numerical1, numerical2);

    // month: Month,
    let year = i16::from_ne_bytes([data[112], data[113]]) % 9998;
    let year = year as i32;

    let month_num = u8::from_ne_bytes([data[114]]) % 13;
    let month_num = month_num.clamp(1, 12);
    let month = Month::try_from(month_num).unwrap();

    let day: u8 = u8::from_ne_bytes([data[115]]) % 32;
    let day: u8 = day.clamp(1, 30);

    let hour: u8 = u8::from_ne_bytes([data[116]]) % 24;
    let minute: u8 = u8::from_ne_bytes([data[117]]) % 60;
    let second: u8 = u8::from_ne_bytes([data[118]]) % 60;
    println!("year: {:?}, month: {:?}, day: {:?}, hour: {:?}, minute: {:?}, second: {:?}", year, month, day, hour, minute, second);

    let res = run(year, month, day, hour, minute, second, weeks, days, hours, minutes, seconds, milliseconds, microseconds, nanoseconds, nanoseconds_i128, nanoseconds_i32, f64_num, f32_num, numerical1, numerical2);
    println!("- result: {:?} [{:?}]", res, data.len());
}

#[test]
fn my_fuzz() {
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

        let weeks = i64::from_ne_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]]) / 604_800;
        let days  = i64::from_ne_bytes([data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]]) / 86_400;
        let hours = i64::from_ne_bytes([data[16], data[17], data[18], data[19], data[20], data[21], data[22], data[23]]) / 3_600;
        let minutes = i64::from_ne_bytes([data[24], data[25], data[26], data[27], data[28], data[29], data[30], data[31]]) / 60;
        let seconds = i64::from_ne_bytes([data[32], data[33], data[34], data[35], data[36], data[37], data[38], data[39]]);
        let milliseconds = i64::from_ne_bytes([data[40], data[41], data[42], data[43], data[44], data[45], data[46], data[47]]);
        let microseconds = i64::from_ne_bytes([data[48], data[49], data[50], data[51], data[52], data[53], data[54], data[55]]);
        let nanoseconds = i64::from_ne_bytes([data[56], data[57], data[58], data[59], data[60], data[61], data[62], data[63]]);
        let nanoseconds_i128 = i128::from_ne_bytes([data[64], data[65], data[66], data[67], data[68], data[69], data[70], data[71], data[72], data[73], data[74], data[75],data[76], data[77], data[78], data[79]]);
        let nanoseconds_i32 = i32::from_ne_bytes([data[80], data[81], data[82], data[83]]);
        let f64_num = f64::from_ne_bytes([data[84], data[85], data[86], data[87], data[88], data[89], data[90], data[91]]);
        let f32_num = f32::from_ne_bytes([data[92], data[93], data[94], data[95]]);
        let numerical1 = f64::from_ne_bytes([data[96], data[97], data[98], data[99], data[100], data[101], data[102], data[103]]);
        let numerical2 = i64::from_ne_bytes([data[104], data[105], data[106], data[107], data[108], data[109], data[110], data[111]]);
        println!("weeks: {:?}\ndays: {:?}\nhours: {:?}\nminutes: {:?}\nseconds: {:?}\nmilliseconds: {:?}\nmicroseconds: {:?}\nnanoseconds: {:?}\nnanoseconds_i128: {:?}\nnanoseconds_i32: {:?}\nf64_num: {:?}\nf32_num: {:?}\nnumerical1: {:?}\nnumerical2: {:?}", weeks, days, hours, minutes, seconds, milliseconds, microseconds, nanoseconds, nanoseconds_i128, nanoseconds_i32, f64_num, f32_num, numerical1, numerical2);

        // month: Month,
        let year = i16::from_ne_bytes([data[112], data[113]]) % 9998;
        let year = year as i32;

        let month_num = u8::from_ne_bytes([data[114]]) % 13;
        let month_num = month_num.clamp(1, 12);
        let month = Month::try_from(month_num).unwrap();

        let day: u8 = u8::from_ne_bytes([data[115]]) % 32;
        let day: u8 = day.clamp(1, 30);

        let hour: u8 = u8::from_ne_bytes([data[116]]) % 24;
        let minute: u8 = u8::from_ne_bytes([data[117]]) % 60;
        let second: u8 = u8::from_ne_bytes([data[118]]) % 60;
        println!("year: {:?}, month: {:?}, day: {:?}, hour: {:?}, minute: {:?}, second: {:?}", year, month, day, hour, minute, second);

        let res = run(
            year, month, day, hour, minute, second,
            weeks, days, hours, minutes, seconds, milliseconds, microseconds, nanoseconds, nanoseconds_i128, nanoseconds_i32, f64_num, f32_num, numerical1, numerical2
        );
        println!("- result: {:?} [{:?}]", res, data.len());
    } else {
        panic!("input data not found");
    }
}

// 39 + 1 fuzz target from bench
fn run(
    year: i32,
    month: Month,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
    milliseconds: i64,
    microseconds: i64,
    nanoseconds: i64,
    _nanoseconds_i128: i128,
    nanoseconds_i32: i32,
    f64_num: f64,
    f32_num: f32,
    numerical1: f64,
    numerical2: i64,
) {
    // my own fuzz target
    if let Ok(date) = Date::from_calendar_date(year, month, day) {
        if let Ok(mut time) = Time::from_hms(hour, minute, second) {
            let offset = UtcOffset::from_whole_seconds(minute as i32 * 60 + minute as i32 ).unwrap_or(UtcOffset::UTC);
            let datetime = date.with_time(time).assume_offset(offset);

            // Perform an offset adjustment that could internally trigger loops for handling DST or large ranges
            let res1 = datetime.to_offset(UtcOffset::UTC);
            println!("  - res1: {:?}", res1);

            // Add large ranges to potentially trigger edge cases in loop-based calculations
            let res2 = datetime.replace_year(year + 10000);  // Very large year increment
            let res3 = datetime.replace_year(year - 10000);  // Very large year decrement
            println!("  - res2: {:?}, res3: {:?}", res2, res3);
            
            // loop
            for new_hour in 0..24 {
                time = time.replace_hour(new_hour).expect("replace_hour failed");
    
                assert_eq!(time.hour(), new_hour);
                assert!(time.minute() == minute);
                assert!(time.second() == second);
            }
        }
    }

    // +1 fuzz target
    let test = Duration::new(seconds, nanoseconds_i32);
    println!("Duration::new {:?}", test);
    let nd1 = numerical1.nanoseconds();
    let nd2 = numerical2.seconds();

    // 5 fuzz targets
    let res1 = nd1.is_zero();
    let res2 = nd1.is_negative();
    let res3 = nd1.is_positive();
    let res4 = nd1.abs();
    let res5 = nd1.unsigned_abs();
    println!("<nd1> res1: {:?}, res2: {:?}, res3: {:?}, res4: {:?}, res5: {:?}", res1, res2, res3, res4, res5);

    let res1 = nd2.is_zero();
    let res2 = nd2.is_negative();
    let res3 = nd2.is_positive();
    let res4 = nd2.abs();
    let res5 = nd2.unsigned_abs();
    println!("<nd2> res1: {:?}, res2: {:?}, res3: {:?}, res4: {:?}, res5: {:?}", res1, res2, res3, res4, res5);

    // 8 existing fuzz targets
    let d1 = Duration::weeks(weeks);
    let d2 = Duration::days(days);
    let d3 = Duration::hours(hours);
    let d4 = Duration::minutes(minutes);
    let d5 = Duration::seconds(seconds);
    let d6 = Duration::milliseconds(milliseconds);
    let d7 = Duration::microseconds(microseconds);
    let d8 = Duration::nanoseconds(nanoseconds);

    // 6 existing fuzz targets
    // let ds1: Duration = Duration::seconds_f64(f64_num);
    // let ds2 = Duration::seconds_f32(f32_num);
    let ds3 = Duration::saturating_seconds_f64(f64_num);
    let ds4 = Duration::saturating_seconds_f32(f32_num);
    let ds5 = Duration::checked_seconds_f64(f64_num);
    let ds6 = Duration::checked_seconds_f32(f32_num);
    println!("ds3: {:?}, ds4: {:?}, ds5: {:?}, ds6: {:?}", ds3, ds4, ds5, ds6);

    // I added this line
    // let d9 = Duration::nanoseconds_i128(nanoseconds_i128);
    let d9 = Duration::nanoseconds_i128(nanoseconds as i128);
    println!("d8: {:?}, d9: {:?}", d8, d9);

    // 7 existing fuzz targets
    assert_eq!(d1.whole_weeks(), weeks);
    assert_eq!(d2.whole_days(), days);
    assert_eq!(d3.whole_hours(), hours);
    assert_eq!(d4.whole_minutes(), minutes);
    assert_eq!(d5.whole_seconds(), seconds);
    // assert_eq!(ds1.as_seconds_f64(), f64_num);
    // assert_eq!(ds2.as_seconds_f32(), f32_num);

    // 2 my fuzz_targets
    assert_eq!(d6.whole_milliseconds(), milliseconds as i128);
    assert_eq!(d7.whole_microseconds(), microseconds as i128);
    // assert_eq!(d8.whole_nanoseconds(), nanoseconds);

    // 5 my fuzz targets
    let tmp1 = (numerical1).seconds().subsec_milliseconds();
    let tmp2 = (numerical1).seconds().subsec_microseconds();
    let tmp3 = (numerical1).seconds().subsec_nanoseconds();
    let tmp4 = (numerical1).microseconds().whole_nanoseconds();
    let tmp5 = (numerical1).nanoseconds().whole_nanoseconds();
    println!("tmp1: {:?}, tmp2: {:?}, tmp3: {:?}, tmp4: {:?}, tmp5: {:?}", tmp1, tmp2, tmp3, tmp4, tmp5);

    // arithmetic
    // FIX: None if overflow
    let dmax = Duration::MAX;
    let dmin = Duration::MIN;
    let dnano1 = (numerical1).nanoseconds();
    let dnano2 = (numerical2).nanoseconds();

    // 2 fuzz target: add
    let add1 = dnano1.saturating_add(dmax);
    let add2 = dnano1.saturating_add(dmin);
    let add3 = dnano2.saturating_add(dmax);
    let add4 = dnano2.saturating_add(dmin);

    let sum1 = dnano1.saturating_add(dnano2);
    let sum2 = dnano2.saturating_add(dnano1);

    println!("dnano1: {:?}, dnano2: {:?}, add1: {:?}, add2: {:?}, add3: {:?}, add4: {:?}", dnano1, dnano2, add1, add2, add3, add4);
    assert_eq!(sum1, sum2);
    if sum1 < dmax {
        assert_eq!(sum1, dnano1.checked_add(dnano2).unwrap());
    }
    let sign1 = dnano1.whole_seconds() == 0 || dnano1.subsec_nanoseconds() == 0 || (dnano1.whole_seconds() > 0 && dnano1.subsec_nanoseconds() > 0) || (dnano1.whole_seconds() < 0 && dnano1.subsec_nanoseconds() < 0);
    let sign2 = dnano2.whole_seconds() == 0 || dnano2.subsec_nanoseconds() == 0 || (dnano2.whole_seconds() > 0 && dnano2.subsec_nanoseconds() > 0) || (dnano2.whole_seconds() < 0 && dnano2.subsec_nanoseconds() < 0);
    assert!(sign1);
    assert!(sign2);
    if dnano1.whole_seconds() > 0 { assert_eq!(dmax.saturating_add(dnano1), dmax);}
    if dnano2.whole_seconds() > 0 { assert_eq!(dmax.saturating_add(dnano2), dmax);}

    // 2 fuzz target: sub
    let sub1 = dnano1.saturating_sub(dmax);
    let sub2 = dnano1.saturating_sub(dmin);
    let sub3 = dnano2.saturating_sub(dmax);
    let sub4 = dnano2.saturating_sub(dmin);
    println!("sub1: {:?}, sub2: {:?}, sub3: {:?}, sub4: {:?}", sub1, sub2, sub3, sub4);

    let diff1 = dnano1.saturating_sub(dnano2);
    let diff2 = dnano2.saturating_sub(dnano1);
    if diff1 < dmax && diff1 > dmin{
        assert_eq!(diff1, dnano1.checked_sub(dnano2).unwrap());
    }
    if diff2 < dmax && diff2 > dmin {
        assert_eq!(diff2, dnano2.checked_sub(dnano1).unwrap());
    }
}

// cargo test --features parsing --features formatting --package time --lib -- fuzz_test::my_fuzz --exact --show-output data=
// "39,57,71,99,78,229,146,251,43,230,224,203,105,124,150,85,250,86,15,98,194,86,37,243,181,6,31,34,198,126,21,130,168,99,16,197,117,166,74,136,117,169,117,156,6,18,53,35,202,225,242,146,34,217,176,88,142,130,130,184,133,134,218,27,107,185,33,243,111,230,54,7,164,123,63,75,233,130,197,217,79,31,156,161,246,68,245,53,140,230,70,129,230,119,18,45,195,129,89,137,67,104,115,192,83,120,121,52,77,57,255,15"
// "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,113,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,205,41,42,43,44,110,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,111,96,97,98,99,100,101,102,103,254,105,106,107,108,109,45,95,112,137"
// "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,104,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,144,85,86,87,88,89,90,91,92,93,94,95,226,97,98,99,100,101,102,103,250,105,106,107,145,109,110,111,112,113"
// "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,74,20,21,22,23,24,25,26,27,28,29,30,31,237,33,34,35,36,37,38,39,104,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,144,111,86,87,88,89,90,91,92,93,94,95,19,97,98,99,100,101,102,103,219,105,106,107,145,109,110,85,112,113"
// "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,112,18,19,20,21,22,23,104,25,26,27,28,219,30,31,32,33,34,35,36,37,38,39,29,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,144,111,86,87,88,89,90,91,92,93,94,95,75,97,98,99,100,101,102,103,200,105,106,107,145,109,110,85,17,113"

// cargo test --features parsing --features formatting --package time --lib -- fuzz_test::my_fuzz --exact --show-output data="246,172,65,190,25,47,159,110,124,132,30,92,37,74,247,20,186,111,242,26,37,119,41,111,174,75,189,37,137,118,194,157,28,4,207,105,198,205,93,188,81,11,158,217,111,5,12,134,228,150,148,35,60,9,229,175,14,64,144,59,81,179,85,53,67,75,40,88,242,177,103,106,84,119,13,208,87,182,79,147,218,128,156,124,34,165,181,124,146,157,77,43,184,48,129,10,91,223,190,106,58,148,127,224,241,234,254,240,193,10,199,236,181,25,170,223,154,131,24,73,27,73,226,219,9,221,106,179"
// cargo test --features parsing --features formatting --package time --lib -- fuzz_test::my_fuzz --exact --show-output data="119,18,236,85,152,117,100,33,40,89,245,169,118,20,106,62,90,238,253,147,111,147,91,92,130,186,218,155,155,148,207,213,238,180,43,73,113,181,37,73,134,245,29,7,12,16,131,242,38,183,230,162,26,143,106,234,121,229,152,32,138,244,137,200,94,248,18,173,94,233,111,54,6,178,214,31,83,4,247,150,28,39,231,0,13,125,114,124,55,244,10,222,66,85,191,27,84,195,242,194,84,88,203,124,7,180,223,174,115,130,88,99,215,162,110,188,179,123,214"