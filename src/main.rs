use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

mod interpreter;
mod server;
mod transpiler;

use interpreter::Interpreter;

#[derive(Parser)]
#[command(name = "panini")]
#[command(about = "🕉️  Panini - Sanskrit programming language with Python-like syntax")]
#[command(version = "0.1.0")]
#[command(author = "Panini Developers")]
#[command(long_about = "
Panini is a Sanskrit programming language that combines the beauty of Devanagari script 
with Python-like syntax. Write code using Sanskrit keywords and execute it seamlessly.

Examples:
  panini                          # Start interactive REPL
  panini run hello.panini         # Run a Sanskrit source file
  panini build hello.panini       # Transpile to Rust and build binary
  panini serve --port 8080        # Start web IDE server
")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start interactive REPL (default)
    #[command(about = "Start interactive Sanskrit REPL")]
    Repl,
    
    /// Run a Panini source file
    #[command(about = "Execute a .panini source file")]
    Run {
        /// Path to .panini source file
        #[arg(help = "Path to the .panini file to execute")]
        file: String,
        
        /// Show detailed execution information
        #[arg(short, long, help = "Enable verbose output")]
        verbose: bool,
    },
    
    /// Build Panini code to Rust binary (transpilation)
    #[command(about = "Transpile .panini code to Rust and build executable")]
    Build {
        /// Path to .panini source file
        #[arg(help = "Path to the .panini file to build")]
        file: String,
        
        /// Output binary name (optional)
        #[arg(short, long, help = "Name of output binary")]
        output: Option<String>,
        
        /// Release mode build
        #[arg(short, long, help = "Build in release mode")]
        release: bool,
    },
    
    /// Start web IDE server
    #[command(about = "Start the web-based Panini IDE")]
    Serve {
        /// Port to run server on
        #[arg(short, long, default_value = "8080", help = "Port number for web server")]
        port: u16,
    },
    
    /// Show example Panini code
    #[command(about = "Display example Sanskrit code")]
    Example,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Repl) => {
            start_repl();
        }
        Some(Commands::Run { file, verbose }) => {
            run_file(&file, verbose);
        }
        Some(Commands::Build { file, output, release }) => {
            build_file(&file, output.as_deref(), release);
        }
        Some(Commands::Serve { port }) => {
            server::start_server(port).await;
        }
        Some(Commands::Example) => {
            show_example();
        }
        None => {
            // Default behavior: start REPL
            start_repl();
        }
    }
}

fn start_repl() {
    print_welcome();
    
    let mut interpreter = Interpreter::default();
    let stdin = io::stdin();

    loop {
        print!("{}", "panini> ".bright_blue().bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0) => {
                // EOF reached (e.g., piped input finished)
                println!("\n{}", "धन्यवाद! Namaste! 🙏".bright_yellow());
                break;
            }
            Ok(_) => {
                let line = input.trim();
                
                if line.is_empty() {
                    continue;
                }
                
                if line == "exit" || line == "quit" || line == "बाहर" {
                    println!("{}", "धन्यवाद! Namaste! 🙏".bright_yellow());
                    break;
                }
                
                if line == "help" || line == "सहायता" {
                    print_repl_help();
                    continue;
                }
                
                if line == "clear" || line == "स्पष्ट" {
                    print!("\x1B[2J\x1B[1;1H"); // Clear screen
                    print_welcome();
                    continue;
                }

                let result = interpreter.run(line);
                if !result.output.is_empty() {
                    print!("{}", result.output);
                }
                if !result.errors.is_empty() {
                    for error in result.errors {
                        println!("{} {}", "त्रुटि:".bright_red().bold(), error);
                    }
                }
            }
            Err(error) => {
                eprintln!("{} {}", "Input error:".red(), error);
                break;
            }
        }
    }
}

fn run_file(file_path: &str, verbose: bool) {
    if !Path::new(file_path).exists() {
        eprintln!("{} File not found: {}", "त्रुटि:".bright_red().bold(), file_path);
        std::process::exit(1);
    }

    if !file_path.ends_with(".panini") {
        eprintln!("{} File should have .panini extension", "चेतावनी:".bright_yellow().bold());
    }

    if verbose {
        println!("{} {}", "▶️  Executing:".bright_green().bold(), file_path);
    }

    match fs::read_to_string(file_path) {
        Ok(source_code) => {
            if verbose {
                println!("{} {} lines", "📄 Source:".bright_blue(), source_code.lines().count());
            }
            
            let mut interpreter = Interpreter::default();
            let result = interpreter.run(&source_code);
            
            if !result.output.is_empty() {
                print!("{}", result.output);
            }
            
            if !result.errors.is_empty() {
                for error in result.errors {
                    eprintln!("{} {}", "त्रुटि:".bright_red().bold(), error);
                }
                std::process::exit(1);
            }
            
            if verbose && result.errors.is_empty() {
                println!("\n{}", "✅ Execution completed successfully".bright_green());
            }
        }
        Err(e) => {
            eprintln!("{} Cannot read file {}: {}", "त्रुटि:".bright_red().bold(), file_path, e);
            std::process::exit(1);
        }
    }
}

fn build_file(file_path: &str, output_name: Option<&str>, release: bool) {
    if !Path::new(file_path).exists() {
        eprintln!("{} File not found: {}", "त्रुटि:".bright_red().bold(), file_path);
        std::process::exit(1);
    }

    let output = output_name.unwrap_or("output");
    
    println!("{} {}", "🔧 Building:".bright_green().bold(), file_path);

    match fs::read_to_string(file_path) {
        Ok(source_code) => {
            match transpiler::transpile_to_rust(&source_code) {
                Ok(rust_code) => {
                    let rust_file = format!("{}.rs", output);
                    if let Err(e) = fs::write(&rust_file, rust_code) {
                        eprintln!("{} Cannot write Rust file: {}", "त्रुटि:".bright_red().bold(), e);
                        std::process::exit(1);
                    }
                    
                    println!("{} Generated: {}", "✅".bright_green(), rust_file);
                    
                    // Compile with rustc
                    let mut cmd = Command::new("rustc");
                    cmd.arg(&rust_file).arg("-o").arg(output);
                    
                    if release {
                        cmd.arg("-O");
                        println!("{} Building in release mode...", "🚀".bright_blue());
                    }
                    
                    match cmd.output() {
                        Ok(output_result) => {
                            if output_result.status.success() {
                                println!("{} Built executable: {}", "🎉".bright_green(), output);
                                // Clean up rust file
                                let _ = fs::remove_file(&rust_file);
                            } else {
                                eprintln!("{} Build failed:", "त्रुटि:".bright_red().bold());
                                eprintln!("{}", String::from_utf8_lossy(&output_result.stderr));
                                std::process::exit(1);
                            }
                        }
                        Err(e) => {
                            eprintln!("{} Failed to run rustc: {}", "त्रुटि:".bright_red().bold(), e);
                            eprintln!("Make sure Rust is installed and rustc is in your PATH");
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{} Transpilation failed: {}", "त्रुटि:".bright_red().bold(), e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("{} Cannot read file {}: {}", "त्रुटि:".bright_red().bold(), file_path, e);
            std::process::exit(1);
        }
    }
}

fn show_example() {
    println!("{}", "📚 Panini Sanskrit Programming Examples".bright_blue().bold());
    println!();
    
    let example_code = r#"!! नमस्ते विश्व - Hello World
दर्श("नमस्ते विश्व")

!! चर और गणना - Variables and Math  
x = 5
y = 10
योग = x + y
दर्श("योग:", योग)

!! शर्त - Conditionals
यदि x < y:
    दर्श("x छोटा है")
अन्यथा:
    दर्श("x बड़ा है")

!! लूप - Loops
यावत् x <= y:
    दर्श(x)
    x = x + 1

!! फंक्शन - Functions
कार्य greet(नाम):
    दर्श("नमस्ते", नाम)

greet("भारत")"#;

    println!("{}", example_code.bright_white());
    println!();
    println!("{}", "💡 Usage:".bright_yellow().bold());
    println!("  {} Save the above code as 'hello.panini'", "1.".bright_cyan());
    println!("  {} Run with: panini run hello.panini", "2.".bright_cyan());
    println!("  {} Build with: panini build hello.panini", "3.".bright_cyan());
}

fn print_welcome() {
    println!("{}", "🕉️  Panini REPL प्रारम्भः".bright_yellow().bold());
    println!("{}", "Sanskrit Programming Language v0.1.0".bright_blue());
    println!("{}", "Type 'help' for commands, 'exit' to quit.".bright_white());
    println!();
}

fn print_repl_help() {
    println!("{}", "📖 REPL Commands:".bright_blue().bold());
    println!("  {} {} - Exit REPL", "exit/quit/बाहर".bright_cyan(), "".bright_white());
    println!("  {} {} - Show this help", "help/सहायता".bright_cyan(), "".bright_white());
    println!("  {} {} - Clear screen", "clear/स्पष्ट".bright_cyan(), "".bright_white());
    println!();
    println!("{}", "🎯 Sanskrit Keywords:".bright_blue().bold());
    println!("  {} {} - Print/Display", "दर्श()".bright_green(), "darsh()".bright_white());
    println!("  {} {} - If condition", "यदि".bright_green(), "yadi".bright_white());
    println!("  {} {} - Else", "अन्यथा".bright_green(), "anyatha".bright_white());
    println!("  {} {} - While loop", "यावत्".bright_green(), "yavat".bright_white());
    println!("  {} {} - For loop", "परिभ्रमण".bright_green(), "paribhraman".bright_white());
    println!("  {} {} - Function", "कार्य".bright_green(), "karya".bright_white());
    println!("  {} {} - Comments", "!!".bright_green(), "".bright_white());
    println!();
}