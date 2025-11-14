use std::time::SystemTime;

fn is_leap_year(year: i32) -> bool {
    // Divisble by 4
    // Unless its divisble by 100 but not 400.
    //
    // Negative, no divisible by 4 and not divisble
    !((year % 4 != 0) || (year % 100 == 0 && year % 400 !=0))
}

// Now make it into a proper struct with year, month, day, hour, minute, second
fn get_current_date() -> 

fn main() {
    // Print current time
    // Now I have the time since unix timestamp.
    let now = SystemTime::now();

    // available as SystemTime::UNIX_EPOCH;
    // 1970-01-01 00:00:00 UTC.
    // So every 60 seconds gets 1 minute
    // every 3600 seconds gets 1 hour
    // every 86400 seconds gets 1 day
    //
    // Then we need to check how many days it is,
    // Let's first calculate without "skottår", we assume its
    // 365 days
    // every 
    let current = now.duration_since(SystemTime::UNIX_EPOCH);

    let seconds_per_day = 86400;
    let seconds_per_hour = 3600;
    let seconds_per_minute = 60;

    let number_of_days = current.clone().expect("Expected seconds").as_secs()/seconds_per_day;
    let hour = (current.clone().expect("Expected seconds").as_secs()%seconds_per_day)/seconds_per_hour;
    let minute = (current.clone().expect("Expected seconds").as_secs()%seconds_per_hour)/seconds_per_minute;
    let second = current.clone().expect("Expected seconds").as_secs()%seconds_per_minute;

    println!("Days that have passed: {:?}, {:?}, {:?}", number_of_days, hour, minute);


    // Now we have number of days passed.
    // We can now start to add them, we need a lookup table for each month

    let mut year = 1970;
    let mut month = 1;
    let mut day = 1;

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

    // I know that its from UNIX_EPOCH.
}
