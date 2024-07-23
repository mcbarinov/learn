use url::Url;

fn main() {
    let url = "mongodb://localhost/my-app";
    let parsed_url = Url::parse(url).unwrap();
    println!("host: {}", parsed_url.host_str().unwrap()); // localhost
    println!("database name: {}", parsed_url.path()); // /my-app
}
