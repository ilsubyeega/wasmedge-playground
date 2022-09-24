use std::io::{self, Error};
use wasmedge_http_req::request;
use wasmedge_wasi_socket::{TcpListener, TcpStream};

fn main() {
    /*
    // thread 'main' panicked at 'no filesystem on wasm'
    try_write_file();
    try_read_file();
    */
    /*
    thread 'main' panicked at 'Failed to execute process: Error { kind: Unsupported, message: "operation not supported on this platform" }
    try_echo();
    */
    run_http_client();
    create_tcp_client();
    create_tcp_server();
}

fn run_http_client() -> io::Result<()> {
    let mut body = Vec::new();
    let res = request::get("http://18.207.88.57:80/get", &mut body)
        .expect("Could not connect to that website.");

    println!("HTTP Status: {} {}", res.status_code(), res.reason());
    Ok(())
}

fn create_tcp_server() {
    let listener = TcpListener::bind("0.0.0.0:1972", true)
        .expect("Couldn't bind to that address: 0.0.0.0:1972");
    println!("Started TCP server.");
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {}
            Err(e) => {}
        }
    }
}

fn create_tcp_client() -> io::Result<()> {
    if let Ok(stream) = TcpStream::connect("45.79.112.203:4242") {
        println!("Connected to the server!");
        stream.shutdown(std::net::Shutdown::Both)
    } else {
        println!("Couldn't connect to server...");
        Err(Error::new(
            io::ErrorKind::BrokenPipe,
            "Couldn't connect to the server",
        ))
    }
}

/*
fn try_write_file() -> io::Result<()> {
    let mut path = env::temp_dir();
    path.set_file_name("rs-test-perms-function.txt");
    std::fs::write(path, "Testing!")
}

fn try_read_file() -> io::Result<()> {
    let mut path = env::temp_dir();
    path.set_file_name("rs-test-perms-function.txt");
    std::fs::read(path).expect("Cannot read the file");
    Ok(())
}

fn try_echo() {
    let result = std::process::Command::new("echo")
        .arg("test")
        .output()
        .expect("Failed to execute process");
    assert!(result.status.success())
}
*/