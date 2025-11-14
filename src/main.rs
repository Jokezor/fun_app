use std::time::SystemTime;

static SECONDS_PER_DAY: i32 = 86400;
static SECONDS_PER_HOUR: i32 = 3600;
static SECONDS_PER_MINUTE: i32 = 60;

fn is_leap_year(year: i32) -> bool {
    // Divisble by 4
    // Unless its divisble by 100 but not 400.
    //
    // Negative, no divisible by 4 and not divisble
    !((year % 4 != 0) || (year % 100 == 0 && year % 400 !=0))
}

// Now make it into a proper struct with year, month, day, hour, minute, second

#[derive(Debug)]
struct Date {
    year: i32,
    month: i32,
    day: i32,
    hour: u64,
    minute: u64,
    second: u64
}

fn get_current_date() -> Date {
    // available as SystemTime::UNIX_EPOCH;
    // 1970-01-01 00:00:00 UTC.
    //
    let now = SystemTime::now();
    let current = now.duration_since(SystemTime::UNIX_EPOCH);

    let number_of_days = current.clone().expect("Expected seconds").as_secs()/SECONDS_PER_DAY;
    let hour = (current.clone().expect("Expected seconds").as_secs()%seconds_per_day)/SECONDS_PER_DAY;
    let minute = (current.clone().expect("Expected seconds").as_secs()%seconds_per_hour)/SECONDS_PER_MINUTE;
    let second = current.clone().expect("Expected seconds").as_secs()%seconds_per_minute;

    let mut year: i32 = 1970;
    let mut month: i32 = 1;
    let mut day: i32 = 1;

    // Create mapping from month to number of days in that month.
    let month_days: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    // Now go through all days, add to start_day and roll over when hitting limit.
    for _i in 0..number_of_days {
        day += 1;
        let mut day_limit = month_days[month-1];

        // Clean up this later.
        if (month == 2 && is_leap_year(year)) {
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


    print!("{year}-{month:0>2}-{day:0>2}, {hour:0>2}:{minute:0>2}:{second:0>2}\n");
    Date { year, month, day, hour, minute, second }

}

fn main() {
    let current_date = get_current_date();

    print!("{:?}", current_date);

}
