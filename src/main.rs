use clap::Parser;
use colored::Colorize;
use spurx_ipcalc::{calculate, display::print_calculation};
use std::io::{self, Write};
use std::process;

#[derive(Parser, Debug)]
#[command(
    name = "spurx-ipcalc",
    author = "Spurx",
    version = "0.1.0",
    about = "A fast, cross-platform CLI subnet calculator for IPv4 and IPv6",
    long_about = "spurx-ipcalc computes network parameters, address ranges, usable hosts, \
binary representations, address scopes, and subnet divisions for IPv4 and IPv6."
)]
struct Cli {
    /// IP address with CIDR prefix (e.g. 192.168.0.0/24, 10.0.0.1, or 2001:db8::/32)
    #[arg(value_name = "CIDR")]
    cidr: Option<String>,

    /// Subnet netmask if IP was provided without CIDR (e.g. 255.255.255.0)
    #[arg(value_name = "NETMASK")]
    netmask: Option<String>,

    /// Split the network into smaller subnets with this new prefix length (e.g. --split 26)
    #[arg(short, long, value_name = "PREFIX")]
    split: Option<u8>,

    /// Output calculation results in JSON format
    #[arg(short, long)]
    json: bool,

    /// Disable colored terminal output
    #[arg(long)]
    no_color: bool,
}

fn main() {
    let args = Cli::parse();

    if args.no_color {
        colored::control::set_override(false);
    }

    let input_cidr = match args.cidr {
        Some(c) => c,
        None => {
            // Interactive prompt
            print!(
                "{}",
                "Enter IP / CIDR network (e.g., 192.168.1.0/24 or 2001:db8::/64): "
                    .bold()
                    .cyan()
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                eprintln!("{}", "Error reading input from stdin.".red());
                process::exit(1);
            }
            let trimmed = input.trim().to_string();
            if trimmed.is_empty() || trimmed == "exit" || trimmed == "q" {
                println!("Exiting.");
                return;
            }
            trimmed
        }
    };

    match calculate(&input_cidr, args.netmask.as_deref(), args.split) {
        Ok(result) => {
            if args.json {
                match serde_json::to_string_pretty(&result) {
                    Ok(json) => println!("{json}"),
                    Err(e) => {
                        eprintln!("{}: Failed to serialize JSON: {e}", "Error".red().bold());
                        process::exit(1);
                    }
                }
            } else {
                print_calculation(&result);
            }
        }
        Err(err) => {
            eprintln!("{}: {err}", "Error".red().bold());
            process::exit(1);
        }
    }
}
