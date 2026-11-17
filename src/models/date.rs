/// `d/m/yy`, in UTC.
pub fn format_date(secs: u64) -> String {
    let (y, m, d) = civil(secs);
    format!("{d}/{m}/{:02}", y % 100)
}

/// `26 August 2026 at 8:33 PM`, in UTC.
pub fn format_long_date(secs: u64) -> String {
    const MONTHS: [&str; 12] = [
        "January", "February", "March", "April", "May", "June", "July", "August", "September",
        "October", "November", "December",
    ];
    let (y, m, d) = civil(secs);
    let minutes = secs / 60 % 60;
    let hour = secs / 3600 % 24;
    let (h12, half) = (if hour % 12 == 0 { 12 } else { hour % 12 }, if hour < 12 { "AM" } else { "PM" });
    format!("{d} {} {y} at {h12}:{minutes:02} {half}", MONTHS[(m - 1) as usize])
}

/// Days-since-epoch to civil date (Howard Hinnant's algorithm).
fn civil(secs: u64) -> (i64, u32, u32) {
    let z = (secs / 86_400) as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = yoe + era * 400 + if m <= 2 { 1 } else { 0 };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_dates() {
        assert_eq!(civil(0), (1970, 1, 1));
        assert_eq!(format_long_date(1_788_035_580), "29 August 2026 at 8:33 PM");
    }
}
