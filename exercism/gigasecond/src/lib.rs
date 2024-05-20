use time::{util, Date, Month, PrimitiveDateTime as DateTime, Time};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond: u32 = 1000000000;
    // let gigasecond: u32 = 370 * 24 * 3600;
    let days: u32 = gigasecond / 24 / 3600;
    let hours: u32 = (gigasecond - days * 24 * 3600) / 3600;
    let minutes: u32 = (gigasecond - days * 24 * 3600 - hours * 3600) / 60;
    let seconds: u32 = gigasecond - days * 24 * 3600 - hours * 3600 - minutes * 60;

    let days: u16 = days.try_into().unwrap();
    let hours: u8 = hours.try_into().unwrap();
    let minutes: u8 = minutes.try_into().unwrap();
    let seconds: u8 = seconds.try_into().unwrap();

    let mut after_sec: u8 = start.second() + seconds;
    let mut after_min: u8 = start.minute() + minutes;
    let mut after_hour: u8 = start.hour() + hours;
    let mut after_day: u16 = u16::from(start.day()) + days;

    if after_sec >= 60 {
        after_sec -= 60;
        after_min += 1;
    }

    if after_min >= 60 {
        after_min -= 60;
        after_hour += 1;
    }

    if after_hour >= 24 {
        after_hour -= 24;
        after_day += 1;
    }

    let mut after_year: i32 = start.year();
    let mut after_month: Month = start.month();

    let mut day_in_month = u16::from(util::days_in_year_month(after_year, after_month));
    while after_day > day_in_month {
        after_day -= day_in_month;
        if after_month == Month::December {
            after_year += 1;
        }
        after_month = after_month.next();
        day_in_month = u16::from(util::days_in_year_month(after_year, after_month));
    }

    dt(
        after_year,
        after_month.try_into().unwrap(),
        after_day.try_into().unwrap(),
        after_hour,
        after_min,
        after_sec,
    )
}

pub fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
    DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}
