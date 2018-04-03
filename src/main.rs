/// Note:
/// We need to install: 'libssl-dev' (CentOS: 'openssl-devel') and 'pkg-config' for reqwest to compile
extern crate reqwest;
extern crate regex;
extern crate clap;
use std::process;
use regex::Regex;
use clap::{Arg, App};

// Define constant array of urls
const URLARR: [&str; 5] = ["https://myexternalip.com/raw",
                           "http://ipinfo.io/ip",
                           "https://ipv4.icanhazip.com/",
                           "http://ipecho.net/plain",
                           "https://canihazip.com/s"];

/// Validate ipv4 address against lazy regex
/// Arguments:
/// ip address as string
/// Return values:
/// true: on valid ipv4 address
/// false: not valid ipv4 address
fn validateip(ipaddr: &str) -> bool {
    // Lazy check of ipv4 against simple regex.
    let reipv4 = Regex::new(r"[0-9]{2,3}\.[0-9]{2,3}\.[0-9]{2}\.[0-9]{1,2}").unwrap();
    // Validate ipv4 against regex
    if reipv4.is_match(ipaddr) {
        return true;
    }
    return false;
}

/// Get external ip from Internet
/// Arguments:
/// An array of urls
/// Return values:
/// tuple with (ipv4_address, answering_url)
fn getextip(urlarr: [&str; 5]) -> (String, String) {
    // Save and return resulting body in this string
    let mut body = String::new();
    let mut success_url = String::new();

    for url in &urlarr {
        let req_url = reqwest::Url::parse(url)
            .expect("Could not parse url");
        // Using match as a try/except solution...
        let mut res = match reqwest::get(req_url) {
            Ok(res) => res,
            Err(_) => {
                continue
            }
        };
        
        if ! res.status().is_success() {
            println!("Got error {}", res.status());
        }
    
        body = res.text().expect("Failed to get body");
        
        // Validate ipv4 against regex
        if ! validateip(&body) {
            continue;
        } else {
            success_url = url.parse().unwrap();
            break;
        }
    }
    return (body, success_url);
}

fn main() {
    let matches = App::new("extip")
                  .version("0.1.1")
                  .author("Magnus Wallin <magnuswallin@tutanota.com>")
                  .about("Gets external ip address")
                  .arg(Arg::with_name("urls")
                       .help("Show the urls used")
                       .long("urls")
                       .short("u"))
                  .get_matches();

    // If flag '-u' is used, show the urls then exit
    if matches.is_present("urls") {
        println!("We check against these urls:");
        for url in &URLARR {
            println!("{}", url);
        }
        println!("The first proper result from any of these urls are used.");
        process::exit(0);
    }

    let (ext_ip, url) = getextip(URLARR);
    if validateip(&ext_ip) {
        println!("Your external ipv4 address is: {}", ext_ip.trim());
        println!("Source: {}", url);
    } else {
        println!("Got no valid ip address ¯\\_(ツ)_/¯. Do you have Internet?");
        process::exit(1);
    }
}
