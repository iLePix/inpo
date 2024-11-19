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
    match get_if_addrs() {
        Ok(interfaces) => {
            for interface in interfaces {
                let ip = interface.addr.ip();
                if !ip.is_ipv4() {
                    continue;
                }
                let class = classify_interface(&interface.name);
                if class == InterfaceClass::Unknown {
                    continue;
                }
                println!("{}: {}", interface.name, ip.to_string().bold().green());
            }
        }
        Err(e) => {
            println!("FAILED TO GET INTERFACES: {}", e.to_string().bold().red());
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum InterfaceClass {
    Wifi,
    Ethernet,
    Unknown,
}

fn classify_interface(interface: &str) -> InterfaceClass {
    match interface {
        i if i.starts_with("wlan") || i.starts_with("wlo") => InterfaceClass::Wifi,
        "wlp82s0" => InterfaceClass::Wifi,
        i if i.starts_with("eth") || i.starts_with("en") => InterfaceClass::Ethernet,
        "enp0s31f6" => InterfaceClass::Ethernet,
        _ => InterfaceClass::Unknown,
    }
}
