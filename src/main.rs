use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};

fn main() {
    let (port, _) = get_args();
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address).expect("Failed to start the server");

    println!("Listening on http://{}",address);

    for request in listener.incoming() {
        let mut stream = request.unwrap();
        
        let (content, status_code) = get_file(handle_request(&stream));
        let mut response = format!(
            "HTTP/2 {}\nContent-Length: {}\n\n",
            status_code,
            content.len()
        ).into_bytes();

        response.extend(content);
        stream.write_all(&response).unwrap();
    }

}

fn get_args() -> (String, String) {
    let args : Vec<String> = env::args().collect();
    let mut port = "3000";
    let mut path = "";
    for arg in args.iter() {
    if arg.starts_with("--port") {
        port = arg.split("=").last().unwrap().trim();
    }
    if arg.starts_with("--rootdir") {
        path = arg.split("=").last().unwrap().trim();
    }
    }
    (port.to_owned(), path.to_owned())
}

fn handle_request(mut stream : &TcpStream) -> String {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    match http_request.first() {
        None => "".to_string(),
        Some(start_line) => {
            match start_line.split(" ").nth(1) {
                None => " ".to_string(),
                Some(path) => return path.to_string()
            }
        }
    }
}

fn get_file(path : String) -> (Vec<u8>, String ){
    let mut full_path = path;

    let (_,rootdir) = get_args();
    if rootdir.is_empty() {
        full_path.remove(0);
    }else {
        full_path = format!("{}{}", rootdir, full_path)
    }
    
    let content = std::fs::read(&full_path);
    match content {
        Ok(content) =>{
            (content, "200 OK".to_string())
        },
        Err(_) => {
            match std::fs::read(format!("{}/404.html", full_path.split("/").nth(0).expect("Somehow it received null"))) {
                Ok(content) => (content, "404 Not Found".to_string()),
                Err(_) => (Vec::from("404 Not Found".as_bytes()), "404 Not Found".to_string())
            }

        }
    }
}