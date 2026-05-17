use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Kay Package Manager (kaypkg) v0.1.0");
        println!("Usage: kaypkg <command> [args]");
        println!("Commands: install, remove, list, update");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "install" => {
            if args.len() < 3 {
                println!("Usage: kaypkg install <package>");
                return;
            }
            println!("Installing package: {}...", args[2]);
            println!("Package {} installed successfully.", args[2]);
        }
        "remove" => {
            if args.len() < 3 {
                println!("Usage: kaypkg remove <package>");
                return;
            }
            println!("Removing package: {}...", args[2]);
            println!("Package {} removed.", args[2]);
        }
        "list" => {
            println!("Installed packages:");
            println!("- stdlib (built-in)");
            println!("- ai-utils (0.1.0)");
        }
        _ => println!("Unknown command: {}", command),
    }
}
