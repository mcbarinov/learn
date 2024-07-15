use humantime::{format_duration, parse_duration};

fn main() {
    // Parse a duration string
    let res = parse_duration("1h20m30s").unwrap();
    dbg!(res); //4830s
    // let res = parse_duration("1hh20mm30ss").unwrap(); can't parse it

    // Format a duration
    let res = format_duration(std::time::Duration::from_secs(4830));
    dbg!(res.to_string()); // 1h 20m 30s
}
