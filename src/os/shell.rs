use crate::run;
use std::{fs, process, env};
use std::path::Path;

pub fn run_shell() {
    println!("KayOS Shell v0.1.0");
    
    if let Ok(boot_script) = fs::read_to_string("boot.ky") {
        let _ = run(boot_script);
    }

    loop {
        let current_dir = env::current_dir().unwrap_or_default();
        print!("kayos:{}> ", current_dir.display());
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0] {
            "exit" => break,
            "ls" => {
                if let Ok(entries) = fs::read_dir(".") {
                    for entry in entries.flatten() {
                        println!("{}", entry.file_name().to_string_lossy());
                    }
                }
            }
            "cat" => {
                if let Some(path) = parts.get(1) {
                    match fs::read_to_string(path) {
                        Ok(content) => println!("{}", content),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
            "ps" => {
                if let Ok(entries) = fs::read_dir("/proc") {
                    for entry in entries.flatten() {
                        if entry.file_name().to_string_lossy().parse::<i32>().is_ok() {
                            println!("{}", entry.file_name().to_string_lossy());
                        }
                    }
                }
            }
            "pwd" => {
                println!("{}", env::current_dir().unwrap().display());
            }
            "cd" => {
                let new_dir = parts.get(1).unwrap_or(&"/");
                if let Err(e) = env::set_current_dir(Path::new(new_dir)) {
                    println!("Error: {}", e);
                }
            }
            "scan" => {
                if parts.len() == 3 {
                    let host = parts[1];
                    let port = parts[2];
                    println!("[*] Scanning {} on port {}...", host, port);
                }
            }
            _ => println!("Unknown command: {}", parts[0]),
        }
    }
}
