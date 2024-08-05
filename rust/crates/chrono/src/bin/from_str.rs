use chrono::DateTime;

fn main() {
    let d = "2024-04-18T05:23:11.888406Z";
    let res = DateTime::parse_from_rfc3339(d).unwrap();
    dbg!(res);
}
