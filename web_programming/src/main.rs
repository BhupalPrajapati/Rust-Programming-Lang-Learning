use std::{
    fs,
    io::{BufRead, BufReader, Lines, Write},
    net::{TcpListener, TcpStream},
    os::windows::thread,
    sync::{Arc, Mutex}, time::Duration,
    thread::sleep,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    // lets store the connection result in a variable in form of Ok and Err variants
    // let stream = listener.accept();

    // note that the value inside the stream is a tuple, we can access the value from it by using index operator

    // println!("The stream is: {:?} \n the socket {:?}", stream.as_ref().unwrap().1, stream.as_ref().unwrap().0);
    // when you open local host 8000 then the sockt Tcpstream is show in the terminal
    // and the server only accepts the incoming requests and not responding anything here

    // i want to n number of listeners

    // if we want to listen only the incoming requests but not try to respond it then use the below loop for that.this logic is try to proceds the incoming request and not responding the naythingd
    /*
        for i in 0..10{
            match listener.accept() {
                Ok((socket,addr))=>println!("The client info is: {:?}", addr),
                Err(e)=> println!("Couldn't get client: {:?}", e),
            }
        }
    */

    // for the multi-threaded purposes
    let mut active_request = Arc::new(Mutex::new(0));

    for stream in listener.incoming() {
        let active_requests = Arc::clone(&active_request); // we are making a clone of the active_request to pass it to the thread
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
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream); // a BufferReader is a struct defined in stnadrad libray for reading out data from the tcp stream

    let lines = buf_reader.lines();
    // here we want to extract the line contains in the http request
    let http_request = lines
        .map(|result| result.unwrap())
        .take_while(|lines| !lines.is_empty())
        .collect::<Vec<String>>();
    print!("Request:{:#?}", http_request);

    // Response Syntax
    /*
    HTTP-version status code Reasone_phrase CRLE
    header-field CRLE
    message-body
     */

    // here we write the response that is responde by the server

    //   let response = "HTTP/1.0 200 OK\r\n";
    //   steam.write(response.as_bytes()).unwrap();
    //   steam.flush().unwrap();   // always remember to call the flush fun on the stream to make sure that all the intermediately buffered content reach the destination

    /*
    // here we write the response according to the HTML page that we are going to display to the user
    let status_line = "HTTP/1.0 200 OK\r\n";
    // here i want to read the content from the index.html and store it into a variable
    let contents = fs::read_to_string("index.html").unwrap(); // we need to bring the relevant module(FS module) into the scope
                                                              // calculate the length of the content and store it in a variable
    let length = contents.len();

    // finally, we will concatenate all the pieces of responses using a format macro
    let response = format!(
        "{}Content-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        status_line, length, contents
    );

    // as expected, we have the status of the line which is given by the variable of status_line

    // next we will the write_all function to write the bytes in the response to the stream
    steam.write_all(response.as_bytes()).unwrap();
    steam.flush().unwrap();
    */

    // here we are try the write the logic for http path, like when you type localhost:8000/page or something like that then it will redirect to you on that pages

    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let request_line = lines.next();
    // if the request is being made for a valid resource, then we will return the resource
    // which is html file and the status line
    let (status_line, file_name) = match request_line.unwrap() {
        Ok(line) => match line.as_str() {
            // we need the couple of unwrap() function bcz it has been wrapped by result and then an option

            // if the request in the route then we need to return the index.html
            // inside the index.html page there are more pages which are being requested
            // finally, if the request is not valid that means index.html file is not exist. then we return 404 html page

            // lets create the arm, the 1st arm is the root file is requested
            "GET /HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("index.html")), // commnication betwn client and server is carried out using different methods

            "GET /index1 HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("index1.html")),

            "GET /index2 HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("index2.html")),
            _ => (Some("HTTP/1.1 NOT FOUND\r\n"), Some("404.html")),
        },
        Err(_) => (None, None),
    };

    // here we are need to return the output of the these requested data by storing in a variable

    let contents = fs::read_to_string(file_name.unwrap()).unwrap();

    let response = format!(
        "{} content-length: {}\r\n{}",
        status_line.unwrap(),
        contents.len(),
        contents
    );

    // let response = format!(
    //     "{}Content-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
    //     status_line,
    //     contents.len(),
    //     contents
    // );

    // next write the response as bytes to the stream and we will use the flush function here to flush the output
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
