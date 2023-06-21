use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};

fn main() {
    let args : Vec<_>= env::args().collect();
    let mut port  = "3000";

    for arg in args.iter() {
        if arg.starts_with("--port") {
            port = arg.split("=").last().unwrap().trim();
        }
    }

    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address).expect("Failed to start the server");
    println!("Listening on http://{}",address);

    for request in listener.incoming() {
        let mut stream = request.unwrap();

        let path = handle_request(&stream);
        let (content, status_code) = get_file(path);
        let response = format!("HTTP/1.1 {}\nContent-Length: {}\n\n {}", status_code, content.len(), content);

        stream.write_all(response.as_bytes()).unwrap();
    }

}

fn handle_request(mut stream : &TcpStream) -> String {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
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

fn get_file(path : String) -> (String, String ){
    let full_path = "Data".to_string() + &path;

    let content = std::fs::read_to_string(full_path);
    match content {
        Ok(content) => (content, "200 OK".to_string()),
        Err(_) => {
            match std::fs::read_to_string("Data/404.html") {
                Ok(content) => (content, "404 Not Found".to_string()),
                Err(_) => ("".to_string(), "404 Not Found".to_string())
            }

        }
    }
}
