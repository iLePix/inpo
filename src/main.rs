use colored::Colorize;
use get_if_addrs::get_if_addrs;
use regex::Regex;
use std::error::Error;

const CHECK_IP_URLS: [&str; 2] = ["https://checkip.amazonaws.com", "https://httpbin.org/ip"];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", "LOCAL IP".bold());
    check_local_ip().await;
    println!("{}", "GLOBAL IPs".bold());
    check_global_ip().await;
    Ok(())
}

async fn check_global_ip() {
    let ip_regex = Regex::new(r"(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();

    for &url in &CHECK_IP_URLS {
        print!("{url}: ");
        let response = match reqwest::get(url).await {
            Ok(resp) => resp,
            _ => {
                println!("{}", "REQUEST FAILED".bold().red());
                continue;
            }
        };
        let text = match response.text().await {
            Ok(txt) => txt,
            _ => {
                println!("{}", "GETTING RESPONSE FAILED".bold().red());
                continue;
            }
        };
        match ip_regex.captures(&text).and_then(|cap| cap.get(0)) {
            Some(ip) => println!("{}", ip.as_str().bold().green()),
            None => println!("{}", "PARSING RESPONSE FAILED".bold().red()),
        }
    }
}

async fn check_local_ip() {
    print!("en0: ");
    match get_if_addrs() {
        Ok(interfaces) => {
            let en0_ip = interfaces
                .iter()
                .filter(|i| i.addr.ip().is_ipv4())
                .find(|i| i.name == "en0")
                .map(|i| i.addr.ip());
            match en0_ip {
                Some(ip) => println!("{}", ip.to_string().bold().green()),
                None => println!("FAILED TO FIND INTERFACE WITH NAME 'en0'"),
            }
        }
        Err(e) => {
            println!("FAILED TO GET INTERFACES: {}", e.to_string().bold().red());
        }
    }
}
