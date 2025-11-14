#![allow(dead_code)]

use std::time::SystemTime;

static SECONDS_PER_DAY: u64 = 86400;
static SECONDS_PER_HOUR: u64 = 3600;
static SECONDS_PER_MINUTE: u64 = 60;

fn is_leap_year(year: i32) -> bool {
    // Divisble by 4
    // Unless its divisble by 100 but not 400.
    //
    // Negative, no divisible by 4 and not divisble
    !((year % 4 != 0) || (year % 100 == 0 && year % 400 !=0))
}


#[derive(Debug)]
struct DateTime {
    year: i32,
    month: i32,
    day: i32,
    hour: u64,
    minute: u64,
    second: u64
}


#[derive(Debug)]
struct Date {
    year: i32,
    month: i32,
    day: i32
}

fn get_current_minute(current_time: u64) -> u64 {
    (current_time%SECONDS_PER_HOUR)/SECONDS_PER_MINUTE
}

fn get_current_hour(current_time : u64) -> u64 {
    (current_time%SECONDS_PER_DAY)/SECONDS_PER_HOUR
}

fn get_current_second(current_time: u64) -> u64 {
    current_time%SECONDS_PER_MINUTE
}

fn get_current_days(current_time: u64) -> u64 {
    current_time/SECONDS_PER_DAY
}


fn get_current_date(current_time: u64) -> Date {
    let number_of_days = get_current_days(current_time);
    let mut year: i32 = 1970;
    let mut month: i32 = 1;
    let mut day: i32 = 1;

    // Create mapping from month to number of days in that month.
    let month_days: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    // Now go through all days, add to start_day and roll over when hitting limit.
    for _i in 0..number_of_days {
        day += 1;
        let mut day_limit = month_days[(month-1) as usize];

        // Clean up this later.
        if month == 2 && is_leap_year(year) {
            day_limit += 1;
        }
        if day > day_limit {
            month += 1;
            day -= day_limit;
        }
        if month > 12 {
            year += 1;
            month = 1;
        }
    }

    Date { year, month, day }
}


fn get_current_datetime() -> DateTime {
    // available as SystemTime::UNIX_EPOCH;
    // 1970-01-01 00:00:00 UTC.
    //
    let now = SystemTime::now();
    let current = now.duration_since(SystemTime::UNIX_EPOCH).expect("Expected seconds").as_secs();

    let hour = get_current_hour(current);
    let minute = get_current_minute(current);
    let second = get_current_second(current);

    let current_date: Date = get_current_date(current);

    DateTime { year: current_date.year, month: current_date.month, day: current_date.day, hour, minute, second }

}

fn main() {
    let current_date = get_current_datetime();

    print!("{:?}\n", current_date);

}
