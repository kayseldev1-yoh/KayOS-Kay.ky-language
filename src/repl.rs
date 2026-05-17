use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use kayky::lexer::Lexer;
use kayky::parser::Parser;
use kayky::runtime::Interpreter;

fn main() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Kay.ky Interactive REPL v0.1.0");
    println!("  Type 'exit' or press Ctrl+C to quit");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let mut rl = DefaultEditor::new().expect("Failed to create editor");
    let mut interpreter = Interpreter::new();

    loop {
        let readline = rl.readline("ky> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                
                if line.trim() == "exit" { break; }
                if line.trim().is_empty() { continue; }

                let mut lexer = Lexer::new(line);
                match lexer.tokenize() {
                    Ok(tokens) => {
                        let mut parser = Parser::new(tokens);
                        match parser.parse() {
                            Ok(stmts) => {
                                if let Err(e) = interpreter.interpret(stmts) {
                                    eprintln!("Runtime Error: {}", e);
                                }
                            }
                            Err(e) => eprintln!("Parser Error: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Lexer Error: {}", e),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
