use chrono::{DateTime, TimeZone, Utc};

fn main() {
    let now: DateTime<Utc> = Utc::now();
    println!("{}", now); // 2020-09-21 06:35:15.767329 UTC

    let some_date: DateTime<Utc> = Utc.with_ymd_and_hms(2020, 1, 21, 22, 12, 59).unwrap();
    println!("{}", some_date); // 2020-01-21 22:02:03 UTC

    let now_delta = Utc::now().checked_sub_signed(chrono::Duration::hours(1)).unwrap();
    println!("{}", now_delta); // 2020-01-21 22:02:03 UTC
}
