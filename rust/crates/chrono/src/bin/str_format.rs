use chrono::Utc;

fn main() {
    let t = Utc::now();
    let str_t = t.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{}", str_t);
}
