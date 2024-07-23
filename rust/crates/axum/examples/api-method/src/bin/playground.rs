fn main() {
    let uri = "/api-post/qwe/123/api-post/eqw";
    let res = uri.replacen("/api-post/", "/api/", 1);
    dbg!(res);
}
