use std::net::{ TcpListener, TcpStream };
use std::io::{ BufRead, BufReader, Write };
use std::fs;
use std::thread;
use std::sync::{ Arc, Mutex };
use std::time::Duration;

pub fn example() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let mut active_requests = Arc::new(Mutex::new(0));

    let stream = listener.accept();

    println!("Stream {:?} \n Socket: {:?}", stream.as_ref().unwrap().1, stream.as_ref().unwrap().0);

    for stream in listener.incoming() {
        let active_requests = Arc::clone(&active_requests);
        let stream = stream.unwrap();

        thread::spawn(move || {
            {
                let mut connection = active_requests.lock().unwrap();
                *connection += 1;
                if *connection >= 3 {
                    thread::sleep(Duration::from_secs(2));
                }
            }

            handle_connection(stream);

            {
                let mut connection = active_requests.lock().unwrap();
                *connection -= 1;
            }
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next();

    let (status_line, file_name) = match request_line.unwrap().unwrap().as_str() {
        "GET / HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("pages/index.html")),
        "GET /page1 HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("pages/page1.html")),
        _ => (Some("HTTP/1.1 404 NOT FOUND\r\n"), Some("pages/404.html")),
    };

    let contents = fs::read_to_string(file_name.unwrap()).unwrap();

    let response = format!(
        "{} Content-Length: {}\r\n\r\n{}",
        status_line.unwrap(),
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
