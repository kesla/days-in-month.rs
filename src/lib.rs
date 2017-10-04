extern crate leap_year;

use leap_year::is_leap_year;

pub fn days_in_month(year: i32, month: u32) -> u32 {
    return match month {
        2 => if is_leap_year(year) { 29 } else { 28 },
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(0, 1), 31);
        assert_eq!(days_in_month(2000, 2), 29);
        assert_eq!(days_in_month(2001, 2), 28);
        assert_eq!(days_in_month(0, 3), 31);
        assert_eq!(days_in_month(0, 4), 30);
        assert_eq!(days_in_month(0, 5), 31);
        assert_eq!(days_in_month(0, 6), 30);
        assert_eq!(days_in_month(0, 7), 31);
        assert_eq!(days_in_month(0, 8), 31);
        assert_eq!(days_in_month(0, 9), 30);
        assert_eq!(days_in_month(0, 10), 31);
        assert_eq!(days_in_month(0, 11), 30);
        assert_eq!(days_in_month(0, 12), 31);
    }
}
