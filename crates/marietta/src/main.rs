use clap::{Parser, Subcommand};
use std::path::PathBuf;

use marietta_compiler::pipeline;

#[derive(Parser)]
#[command(name = "marietta", about = "The Marietta compiler", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check a Marietta source file for errors without generating code.
    Check {
        /// Source file to check.
        file: PathBuf,
        /// Dump Marietta IR.
        #[arg(long)]
        dump_ir: bool,
        /// Dump Cranelift IR.
        #[arg(long)]
        dump_clir: bool,
        /// Dump generated assembly.
        #[arg(long)]
        dump_asm: bool,
    },
    /// Compile a Marietta source file to a native object file.
    Build {
        /// Source file to compile.
        file: PathBuf,
        /// Output object file path (defaults to <file>.o).
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Dump Marietta IR.
        #[arg(long)]
        dump_ir: bool,
        /// Dump Cranelift IR.
        #[arg(long)]
        dump_clir: bool,
        /// Dump generated assembly.
        #[arg(long)]
        dump_asm: bool,
    },
    /// Compile and JIT-execute a Marietta source file.
    Run {
        /// Source file to run.
        file: PathBuf,
        /// Dump Marietta IR.
        #[arg(long)]
        dump_ir: bool,
        /// Dump Cranelift IR.
        #[arg(long)]
        dump_clir: bool,
        /// Dump generated assembly.
        #[arg(long)]
        dump_asm: bool,
    },
}

fn read_source(file: &PathBuf) -> Option<String> {
    match std::fs::read_to_string(file) {
        Ok(s)  => Some(s),
        Err(e) => { eprintln!("error: cannot read {}: {e}", file.display()); None }
    }
}

/// Load the standard library and prepend it to the user's source code.
///
/// The std module provides core traits and functions available in all programs.
/// It's automatically included before the user's code, making all std definitions
/// available without explicit imports.
fn load_with_stdlib(user_source: &str) -> String {
    // Try to find std.ma in various locations:
    // 1. Relative to the executable (../crates/marietta/std.ma)
    // 2. Relative to current directory (./crates/marietta/std.ma)
    
    let exe_path = std::env::current_exe().ok();
    let mut std_paths = vec![];
    
    if let Some(exe) = exe_path {
        // Build directory structure: target/debug/marietta -> crates/marietta/std.ma
        if let Some(parent) = exe.parent() {
            std_paths.push(parent.parent().map(|p| p.join("crates/marietta/std.ma")));
        }
    }
    
    // Also try from current working directory
    std_paths.push(Some(std::path::PathBuf::from("crates/marietta/std.ma")));
    std_paths.push(Some(std::path::PathBuf::from("./std.ma")));
    
    let mut std_source = String::new();
    for path_opt in std_paths {
        if let Some(path) = path_opt {
            if let Ok(contents) = std::fs::read_to_string(&path) {
                std_source = contents;
                break;
            }
        }
    }
    
    if std_source.is_empty() {
        eprintln!("warning: could not load standard library");
    }
    
    // Combine std library with user source, separated by newlines
    format!("{}\n\n# User code:\n{}", std_source, user_source)
}

fn print_result(result: &pipeline::PipelineResult) {
    for msg in &result.diagnostics {
        eprintln!("{msg}");
    }
}

fn main() {
    let cli = Cli::parse();

    let success = match cli.command {
        Command::Check { file, dump_ir, dump_clir, dump_asm } => {
            let Some(source) = read_source(&file) else { std::process::exit(1) };
            let source_with_std = load_with_stdlib(&source);
            let opts = pipeline::DumpOptions {
                dump_ir,
                dump_clir,
                dump_asm,
            };
            let result = pipeline::check(&source_with_std, &opts);
            print_result(&result);
            result.success
        }

        Command::Build { file, output, dump_ir, dump_clir, dump_asm } => {
            let Some(source) = read_source(&file) else { std::process::exit(1) };
            let source_with_std = load_with_stdlib(&source);
            let name   = file.file_stem().unwrap_or_default().to_string_lossy().into_owned();
            let output = output.unwrap_or_else(|| file.with_extension("o"));
            let opts = pipeline::DumpOptions {
                dump_ir,
                dump_clir,
                dump_asm,
            };
            let result = pipeline::build(&source_with_std, &name, &output, &opts);
            print_result(&result);
            result.success
        }

        Command::Run { file, dump_ir, dump_clir, dump_asm } => {
            let Some(source) = read_source(&file) else { std::process::exit(1) };
            let source_with_std = load_with_stdlib(&source);
            let opts = pipeline::DumpOptions {
                dump_ir,
                dump_clir,
                dump_asm,
            };
            let result = pipeline::run(&source_with_std, &opts);
            print_result(&result);
            result.success
        }
    };

    if !success {
        std::process::exit(1);
    }
}
