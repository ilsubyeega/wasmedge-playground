use wasmedge_http_req::request;

fn main() {
    let mut writer = Vec::new();
    let res = request::get("http://1.1.1.1/", &mut writer).unwrap();
    println!("Status: {} {}", res.status_code(), res.reason());
}