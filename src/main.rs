use clap::{Parser, Subcommand};
use lapzt::{Program, ProgramError};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(name = "lapzt", about = "Lap Zig Transpiler")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "Compile Lap source files to Zig")]
    Compile {
        #[arg(
            help = "Lap source files, concatenated in argument order",
            required = true
        )]
        files: Vec<PathBuf>,
        #[arg(long, help = "Disable label erasure")]
        no_erasure: bool,
    },
    #[command(about = "Check Lap source files for errors")]
    Check {
        #[arg(
            help = "Lap source files, concatenated in argument order",
            required = true
        )]
        files: Vec<PathBuf>,
    },
    #[command(about = "Print the liblapzt standard library")]
    Std,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Compile { files, no_erasure } => compile(&files, no_erasure),
        Command::Check { files } => check(&files),
        Command::Std => {
            print!("{}", lapzt::std());
            ExitCode::SUCCESS
        }
    }
}

fn compile(files: &[PathBuf], no_erasure: bool) -> ExitCode {
    let program = match load(files) {
        Ok(program) => program,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };
    let emitted = if no_erasure {
        program.emit_without_erasure()
    } else {
        program.emit()
    };
    match emitted {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(errors) => {
            report(&errors);
            ExitCode::FAILURE
        }
    }
}

fn check(files: &[PathBuf]) -> ExitCode {
    let program = match load(files) {
        Ok(program) => program,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };
    let errors = program.errors();
    if errors.is_empty() {
        ExitCode::SUCCESS
    } else {
        report(&errors);
        ExitCode::FAILURE
    }
}

fn load(files: &[PathBuf]) -> Result<Program, String> {
    let source = files
        .iter()
        .map(|file| {
            fs::read_to_string(file).map_err(|error| format!("{}: {error}", file.display()))
        })
        .collect::<Result<Vec<String>, String>>()?
        .join("\n");
    Program::parse(&source).map_err(|error| error.to_string())
}

fn report(errors: &[ProgramError]) {
    for error in errors {
        eprintln!("{error}");
    }
}
