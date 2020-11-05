use structopt::StructOpt;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::net::TcpStream;
use std::time::{Instant};

#[derive(StructOpt)]
struct Cli {
    #[structopt(help = "An url endpoint to which the requests must be made")]
    url: String,
    #[structopt(help = "An interger value indicating the number times <url> needs to be hit")]
    profile: u16
}


fn main() {
    let _args = Cli::from_args();
    let mut _counter: u32 = 0;
    let mut url: String;
    let mut response_values:Vec<u128> = Vec::new();
    let mut response_size:Vec<usize> = Vec::new();
    
    for _i in 0.._args.profile{
        url = _args.url.clone();
        let now = Instant::now();
        let val = connect(url);
        let new_now = Instant::now();
        response_values.push(new_now.duration_since(now).as_millis());
        response_size.push(val.unwrap());
        _counter = _counter + 1;
    }

    println!("Number of Requests --> {}", _args.profile);
    match response_values.iter().min() {
        Some(min) => println!("The Fastest response time --> {:?}ms", min),
        None => println!("Empty vector"),
    }

    match response_values.iter().max() {
        Some(min) => println!("The Slowest response time--> {:?}ms", min),
        None => println!("Empty vector"),
    }


    match response_size.iter().min() {
        Some(min) => println!("The Size in bytes of the smallest Response --> {}", min),
        None => println!("Empty vector"),
    }

    match response_size.iter().max() {
        Some(min) => println!("The Size in bytes of the largest Response --> {}", min),
        None => println!("Empty vector"),
    }

   
    println!("The Mean Time --> {}ms", mean(&response_values));
    println!("The Median Time --> {}ms", median(&response_values));
    println!("The Percentage of Requests succeded --> {}%", (_counter as f32/ _args.profile as f32)*100.0);
}

fn mean(response_values: &[u128]) -> f64 {
    let sum: u128 = Iterator::sum(response_values.iter());
    sum as f64 / (response_values.len() as f64)
}

fn median(list: &[u128]) -> f64 {
    let len = list.len();
    let mid = len / 2;
    if len % 2 == 0 {
        mean(&list[(mid - 1)..(mid + 1)])
    } else {
        list[mid] as f64
    }
}

fn connect(_value: String) -> Result<usize> {
    let split = _value.split("/");
    let vec: Vec<&str> = split.collect();

    let mut host: String = vec[2].to_owned();
    let port: &str = ":80";

    host.push_str(port);
    let mut stream = TcpStream::connect(host)?;

    let mut request_data = String::new();
    request_data.push_str("GET /");
    request_data.push_str(vec[3]);
    request_data.push_str(" HTTP/1.0");
    request_data.push_str("\r\n");
    request_data.push_str(" -H 'Accept: application/json'");
    request_data.push_str("\r\n");
    request_data.push_str(" -H 'Content-Type: application/json'");
    request_data.push_str("\r\n");
    request_data.push_str("Host: ");
    request_data.push_str(vec[2]);
    request_data.push_str("\r\n");
    request_data.push_str("Connection: close"); // <== Here!
    request_data.push_str("\r\n");
    request_data.push_str("\r\n");
    if let Err(request) = stream.write_all(request_data.as_bytes()){
        return Err(request)
    }

    let mut buf = String::new();
    let result = stream.read_to_string(&mut buf)?;

    println!("buf = {}", buf);

    Ok(result)
}