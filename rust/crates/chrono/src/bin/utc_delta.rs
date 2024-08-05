use chrono::{DateTime, Utc};

fn utc_delta_seconds(value: i64) -> DateTime<Utc> {
    Utc::now().checked_add_signed(chrono::Duration::seconds(value)).unwrap()
}

fn main() {
    println!("{}", Utc::now());
    println!("{}", utc_delta_seconds(600));
    println!("{}", utc_delta_seconds(-600));
}
