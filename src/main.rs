use std::time;
use std::thread;
use std::process;
use clap::Parser;

//TODO: Make sure localhost and 127.0.0.1 work!

// Running `target/debug/waiturl 'http://localhost:8080/'`
// thread 'main' panicked at src/main.rs:93:7:
// Error during request : reqwest::Error { kind: Request, url: "http://localhost:8080/", source: hyper_util::client::legacy::Error(Connect, ConnectError("tcp connect error", Os { code: 111, kind: ConnectionRefused, message: "Connection refused" })) }

//TODO: Use stderr for printing errors

//TODO: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch09-02-recoverable-errors-with-result.html

//TODO Create an enum of exit codes
//https://blog.rust-lang.org/2022/05/19/Rust-1.61.0.html

// TODO Find out how to write to stdout!
// TODO Write error messages to stderr?

/// waiturl: Wait for a specific HTTP response by continuously polling a URL
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
  /// The URL to poll
  url: String,

  /// The HTTP response status code to wait for, defaults to "200 OK"
  #[arg(short, long, default_value_t = 200)]
  response_code: u16,

  /// Timeout in seconds until the program terminates, runs infinitely otherwise
  #[arg(short, long, default_value_t = 0)]
  timeout: u64,

  /// The interval in seconds between each poll request. This does not include the time for the actual request
  #[arg(short, long, default_value_t = 1)]
  interval: u64,

  /// Suppress any output. The program's success is returned as exit code "0", other values indicate timeouts or errors
  #[arg(short, long, default_value_t = false)]
  quiet: bool
}

// TODO https://stackoverflow.com/questions/42917566/what-is-this-question-mark-operator-about
// TODO Use some red color for errors!

fn main() {
  let arguments = Arguments::parse();
  validate_url(&arguments.url);

  let interval_duration_seconds = time::Duration::from_secs(arguments.interval);
  let start_time = time::Instant::now();

  loop {
    // TODO Rather return a result and process everything here, prevent deep function nesting
    poll(&arguments.url, arguments.response_code, arguments.quiet);

    if arguments.timeout > 0 && start_time.elapsed().as_secs() >= arguments.timeout {
      println!("Timeout after {} seconds!", arguments.timeout);
      process::exit(1);
    }

    thread::sleep(interval_duration_seconds);
  };
}

// TODO This does absolutely nothing!
// Maybe use reqwest, maybe the Client has some built-in validation
fn validate_url(url: &String){
  match reqwest::Url::parse(url) {
    Ok(_url) => (),
    Err(_error) => {
      // TODO Use stderr
      println!("Error: The URL cannot be parsed!");
      process::exit(1);
    }
  };
}

// TODO Use different library for making requests?
fn poll(url: &String, expected_response_code: u16, is_quiet: bool) {
  //TODO use same client for each request
  // let client = reqwest::Client::new();

  // let response = reqwest::blocking::get("http://httpbin.org/get")?;

  // https://docs.rs/reqwest/latest/reqwest/blocking/index.html

  //TODO Do not use unwrap and use Result<Response, Error>
  // https://rust-classes.com/chapter_3_3
  // https://doc.rust-lang.org/rust-by-example/std/result.html
  let response: reqwest::blocking::Response = match reqwest::blocking::get(url) {
    Ok(response) => response,
    Err(error) => {
      panic!("Error during request : {:?}", error)
    }
  };
  let actual_response_code: u16 = response.status().as_u16();

  check_result(url, expected_response_code, actual_response_code, is_quiet);
}

fn check_result(url: &String, expected_response_code: u16, actual_response_code: u16, is_quiet: bool) {
  if actual_response_code == expected_response_code {
    print_result_success(url, actual_response_code, is_quiet);
    process::exit(0);
  } else {
    print_result_fail(url, expected_response_code, actual_response_code, is_quiet);
  }
}

fn print_result_success(url: &String, actual_response_code: u16, is_quiet: bool) {
  if !is_quiet {
    //TODO print run time in seconds
    println!("{}: {} SUCCESS", url, actual_response_code);
  }
}

fn print_result_timeout() {
  unimplemented!()
}

fn print_result_fail(url: &String, expected_response_code: u16, actual_response_code: u16, is_quiet: bool) {
  if !is_quiet {
    println!("{}: {}, expected {}", url, actual_response_code, expected_response_code);
  }
}

#[allow(dead_code)]
fn playground(){
  println!("Hello, world!");

  let message = "Hello, world! Again!";
  println!("{}", message);

  dbg!(message);

  for _ in 0..2 {
  }
}
