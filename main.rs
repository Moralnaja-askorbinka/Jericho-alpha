use std::{io::{self, Write}};
use chrono::prelude::*;
use get_if_addrs::get_if_addrs;
use std::process::Command;
use reqwest::blocking;
use std::net::{SocketAddr, ToSocketAddrs};
use ipgeolocate::{Locator, Service};

fn get_open_ports() {
    println!("Fetching open TCP and UDP ports...");

    // Fetch TCP ports using `ss` (Linux) or `netstat` (Windows/Mac/Linux)
    #[cfg(target_os = "linux")]
    let tcp_output = Command::new("ss").args(&["-tuln"]).output();

    #[cfg(target_os = "windows")]
    let tcp_output = Command::new("netstat").args(&["-an"]).output();

    #[cfg(target_os = "macos")]
    let tcp_output = Command::new("netstat").args(&["-anv"]).output();

    match tcp_output {
        Ok(output) => {
            println!("TCP and UDP ports:\n");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(e) => eprintln!("Failed to fetch open ports: {}", e),
    }
}

fn get_mac_address() {
    // Linux/Mac: Use `ip addr` to get MAC addresses
    #[cfg(target_os = "linux")]
    #[cfg(target_os = "macos")]
    let output = Command::new("ip").args(&["addr", "show"]).output();

    // Windows: Use `getmac` to get MAC addresses
    #[cfg(target_os = "windows")]
    let output = Command::new("getmac").output();

    match output {
        Ok(output) => {
            println!("MAC Address Information:\n");
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(e) => eprintln!("Failed to fetch MAC address: {}", e),
    }
}

fn get_dmz_info() {
    // Check the default gateway (potentially leading to a DMZ)
    println!("Default Gateway:");
    match Command::new("ip").args(&["route", "show", "default"]).output() {
        Ok(output) => println!("{}", String::from_utf8_lossy(&output.stdout)),
        Err(e) => eprintln!("Failed to fetch default gateway: {}", e),
    }

    // Check the current network interfaces
    println!("\nNetwork Interfaces:");
    match Command::new("ip").args(&["addr", "show"]).output() {
        Ok(output) => println!("{}", String::from_utf8_lossy(&output.stdout)),
        Err(e) => eprintln!("Failed to fetch network interfaces: {}", e),
    }

    // Print system-specific instructions
    println!("\nNote: To check DMZ settings, inspect your router's configuration.");
}

fn get_wan_info() -> Result<(), Box<dyn std::error::Error>> {
    let response = blocking::get("https://api.ipify.org")?;
    let wan_ip = response.text()?.trim().to_string();

    let wan_type = match sys_info::os_type() {
        Ok(os_type) => format!("{} (assumed based on OS)", os_type),
        Err(_) => "Unknown".to_string(),
    };

    println!("WAN IP: {}", wan_ip);
    println!("WAN Type: {}", wan_type);

    Ok(())
}

fn run_programm(prog: &str)
{
        
        let program: &str = prog; 
        let args: Vec<String> = vec![];    
    
        println!("Starting program: {}", program);

        match Command::new(program).args(&args).spawn() {
            Ok(mut child) => {
                println!("Program started successfully, waiting for it to finish...");

                match child.wait() {
                    Ok(status) => println!("Program exited with status: {}", status),
                    Err(e) => eprintln!("Failed to wait for program: {}", e),
                }
            }
            Err(e) => eprintln!("Failed to start program: {}", e),
        }
    
}

fn get_dns() {
    fn resolve_dns(host: &str) -> Option<Vec<SocketAddr>> {
        // Primary DNS resolution
        let addrs: Vec<_> = (host, 0).to_socket_addrs().ok()?.collect();
        if !addrs.is_empty() {
            return Some(addrs);
        }

        // Alternate DNS resolution
        // Attempting to resolve using a hardcoded alternate DNS server
        // Example: using Google's DNS (8.8.8.8) for resolution
        let alternate_dns = "8.8.8.8";
        let alternate_host = format!("{}:{}", alternate_dns, 53); // Port 53 for DNS

        let addrs_alt: Vec<_> = (alternate_host.as_str(), 0).to_socket_addrs().ok()?.collect();
        if !addrs_alt.is_empty() {
            return Some(addrs_alt);
        }

        None
    }

    if let Some(addresses) = resolve_dns("example.com") {
        for addr in addresses {
            println!("Resolved address: {}", addr);
        }
    } else {
        eprintln!("Failed to resolve DNS");
    }
}

fn get_info_os()
{
    match sys_info::os_type() {
        Ok(os_type) => println!("OS Type: {}", os_type),
        Err(e) => println!("Error: {}", e),
    }
    println!("Architecture: {}", std::env::consts::ARCH);

    match sys_info::os_release() {
        Ok(os_release) => println!("OS Release: {}", os_release),
        Err(e) => println!("Error: {}", e),
    }

    match sys_info::cpu_num() {
        Ok(cpu_num) => println!("Number of CPUs: {}", cpu_num),
        Err(e) => println!("Error: {}", e),
    }

    match sys_info::hostname() {
        Ok(hostname) => println!("Hostname: {}", hostname),
        Err(e) => println!("Error: {}", e),
    }
}

fn get_date()
{
    let local_time = Local::now();
    let ymd = local_time.format("%Y-%m-%d").to_string();
    println!("{}", ymd);
}

fn get_time()
{
    let local_time = Local::now();
    let hms = local_time.format("%H:%M:%S").to_string();
    println!("{}", hms);
}

fn get_ip()
{
    if let Ok(interfaces) = get_if_addrs() {
        for iface in interfaces {
            println!("Interface: {}, IP: {}", iface.name, iface.ip());
        }
    } else {
        println!("Failed to get network interfaces.");
    }
}

fn v_pr() {
    println!(
        "\n ╔To see a World in a Grain of Sand and a Heaven in a Wild Flower╗\n \
         ╠Hold Infinity in the palm of your hand═════════════════════════╣\n \
         ╚and Eternity in an hour════════════════════════════════════════╝"
    );
}


fn main() {
    v_pr();
    let all_requests_for_show: Vec<String> = vec!["everything".to_string(), "ip".to_string(), "date".to_string(), "time".to_string(), "os".to_string(), "DNS".to_string(), "WAN".to_string(), "DMZ".to_string(), "MAC".to_string(), "open_ports".to_string()];
    let all_commands: Vec<String> = vec!["help".to_string(), "show".to_string(), "run".to_string(), "end".to_string(), "stop".to_string()];
    let all_commands_discription: Vec<String> = vec!["help".to_string(), "run - any name of programm".to_string(),"show - everything, ip, date, time, os, DNS, WAN, DMZ, MAC, open_ports".to_string(), "end, stop, off - stop running Jericho".to_string()];
    let mut input: String = String::new();
    while input.trim() != "end" && input.trim() != "stop" && input.trim() != "off" {
        println!("");
        input = String::new();
        while input.trim().is_empty() {
            print!(">");
            io::stdout().flush().expect("Failed to flush stdout");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
        }

        let input: &str = input.trim();
        let splited_input: Vec<&str> = input.split_whitespace().collect();
        if !(all_commands.contains(&splited_input[0].to_string())) {
            println!("Unknown command -> {}\nFor more command try 'help'", splited_input[0]);
        }
        if splited_input[0] == "show" {
            if splited_input.len() > 1 {
                let mut requests: Vec<&str> = splited_input.clone();
                requests.remove(0);
                if requests[0] == "everything" {
                    requests = all_requests_for_show.iter().map(|s| s.as_str()).collect();
                }
                for request in &requests
                {
                    if *request == "ip" {
                        get_ip();
                    }
                    if *request == "date" {
                        get_date();
                    }
                    if *request == "time" {
                        get_time();
                    }
                    if *request == "os" {
                        get_info_os();
                    }
                    if *request == "DNS" {
                        get_dns();
                    }
                    if *request == "DMZ" {
                        get_dmz_info();
                    }
                    if *request == "MAC" {
                        get_mac_address();
                    }
                    if *request == "open_ports" {
                        get_open_ports();
                    }
                    if *request == "WAN" {
                        let _ = get_wan_info();
                    }
                    if !(all_requests_for_show.contains(&request.to_string()))
                    {
                        println!("ERROR: didn't find argument -> {}", *request);
                    }
                    println!("");
                }
            } else {
                println!("must be argument after show");
            }
        }

        if splited_input[0] == "help" {
            for command in &all_commands_discription
            {
                println!("{}",command);
            }
        }

        if splited_input[0] == "run" {
            if splited_input.len() > 1 {
                run_programm(splited_input[1]);
            } else {
                println!("must be argument after run");
            }
        }
    }
}
