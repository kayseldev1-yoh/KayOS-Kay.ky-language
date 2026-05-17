use std::env;
use kayky::os::shell::run_shell;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        // Mode OS: Masuk ke KayOS Shell
        run_shell();
    } else if args[1] == "tool" {
        // Mode Tool: Eksekusi tool keamanan (misal: kayky tool scanner)
        println!("Running security tool: {}", args[2]);
    } else {
        println!("Usage: kayky [tool name]");
    }
}
