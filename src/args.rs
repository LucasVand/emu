use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "emu")]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
#[derive(Debug, Clone)]
pub struct AsmArgs {
    #[command(subcommand)]
    pub command: AsmCommand,
}

#[derive(Debug, Clone, Subcommand)]
pub enum AsmCommand {
    /// Assemble an assembly file into binary
    Assemble {
        /// Input assembly file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Output binary file
        #[arg(value_name = "FILE", default_value = "./bin/out.bin")]
        output: PathBuf,

        /// Standard library path
        #[arg(long, default_value = "./asm/std")]
        std_path: PathBuf,
    },
    /// Disassemble a binary file
    Disassemble {
        /// Input binary file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Show detailed disassembly information
        #[arg(short, long)]
        detailed: bool,
    },
    /// Assemble and run a file
    Run {
        /// Input assembly file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Standard library path
        #[arg(long, default_value = "./asm/std")]
        std_path: PathBuf,

        /// Enable graphics window
        #[arg(short = 'g', long)]
        graphics: bool,

        /// Execution speed in microseconds per instruction
        #[arg(long, short = 's', default_value = "0")]
        speed: usize,

        /// Verbosity level (0=silent, 1=minimal, 2=normal, 3=verbose)
        #[arg(short = 'v', long, default_value = "2")]
        verbose: u8,

        /// Interactive mode - press space to execute next instruction
        #[arg(short = 'i', long)]
        interactive: bool,
    },
    /// Execute a binary in emulator
    Execute {
        /// Binary file to run
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Enable graphics window
        #[arg(short = 'g', long)]
        graphics: bool,

        /// Execution speed in microseconds per instruction
        #[arg(long, default_value = "0")]
        speed: usize,

        /// Verbosity level (0=silent, 1=minimal, 2=normal, 3=verbose)
        #[arg(short = 'v', long, default_value = "2")]
        verbose: u8,

        /// Interactive mode - press space to execute next instruction
        #[arg(short = 'i', long)]
        interactive: bool,
    },
}
