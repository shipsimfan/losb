use std::time::SystemTime;

const MS_DOS_CONVERSION: u128 = 8 * YEAR_MS + 2 * LEAP_YEAR_MS;

const YEAR_MS: u128 = 365 * DAY_MS;
const LEAP_YEAR_MS: u128 = 366 * DAY_MS;
const DAY_MS: u128 = 24 * HOUR_MS;
const HOUR_MS: u128 = 60 * MINUTE_MS;
const MINUTE_MS: u128 = 60 * SECOND_MS;
const SECOND_MS: u128 = 1000;

const YEAR_MONTH_LENGTHS: [u128; 12] = [
    31 * DAY_MS,
    28 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
];
const LEAP_YEAR_MONTH_LENGTHS: [u128; 12] = [
    31 * DAY_MS,
    29 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
    30 * DAY_MS,
    31 * DAY_MS,
];

pub fn generate_date_time(time: SystemTime) -> Option<(u16, u16, u8)> {
    let mut millis = match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_millis() - MS_DOS_CONVERSION,
        Err(_) => return None,
    };

    let year = calculate_year(&mut millis);
    let month = calculate_month(&mut millis, year);
    let day = calculate_simple(&mut millis, DAY_MS) + 1;

    let hour = calculate_simple(&mut millis, HOUR_MS);
    let minute = calculate_simple(&mut millis, MINUTE_MS);
    let second = calculate_simple(&mut millis, 2 * SECOND_MS);

    let tenth = calculate_simple(&mut millis, SECOND_MS / 10) as u8;

    let date =
        (day as u16 & 0x1F) | (month as u16 & 0x0F) << 5 | ((year - 1980) as u16 & 0x7F) << 9;
    let time = (second as u16 & 0x1F) | (minute as u16 & 0x1F) << 5 | (hour as u16 & 0x1F) << 10;

    Some((date, time, tenth))
}

fn calculate_year(millis: &mut u128) -> usize {
    let mut year = 1980;
    loop {
        let leap_year = is_leap_year(year);

        let year_millis = if leap_year { LEAP_YEAR_MS } else { YEAR_MS };

        if *millis < year_millis {
            return year;
        }

        *millis -= year_millis;
        year += 1;
    }
}

fn calculate_month(millis: &mut u128, year: usize) -> usize {
    let month_lengths = if is_leap_year(year) {
        LEAP_YEAR_MONTH_LENGTHS
    } else {
        YEAR_MONTH_LENGTHS
    };

    let mut month = 0;
    loop {
        if *millis < month_lengths[month] {
            return month + 1;
        }

        *millis -= month_lengths[month];
        month += 1;
    }
}

fn calculate_simple(millis: &mut u128, unit: u128) -> usize {
    let value = (*millis / unit) as usize;
    *millis = *millis % unit;
    value
}

fn is_leap_year(year: usize) -> bool {
    if year % 4 != 0 {
        return false;
    }

    if year % 400 == 0 {
        return true;
    }

    year % 100 != 0
}
