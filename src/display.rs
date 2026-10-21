use crate::types::{CalculationResult, Ipv4CalculationResult, Ipv6CalculationResult};
use colored::Colorize;

pub fn print_calculation(result: &CalculationResult) {
    match result {
        CalculationResult::V4(v4) => print_ipv4(v4),
        CalculationResult::V6(v6) => print_ipv6(v6),
    }
}

fn print_header(title: &str) {
    println!("\n{}", format!("=== {title} ===").bold().cyan());
}

fn print_row(label: &str, value: &str) {
    println!("  {:<22} : {}", label.bright_black(), value.bold());
}

fn print_ipv4(res: &Ipv4CalculationResult) {
    print_header("IPv4 Subnet Calculation");
    print_row("Input", &res.input);
    print_row(
        "CIDR Prefix",
        &format!("/{}", res.prefix_len).bright_yellow(),
    );
    print_row("Network Address", &res.network_address.to_string().green());
    print_row(
        "Broadcast Address",
        &res.broadcast_address.to_string().green(),
    );
    print_row("Subnet Netmask", &res.netmask.to_string());
    print_row("Wildcard (Hostmask)", &res.wildcard_mask.to_string());

    print_header("Capacity & Host Range");
    if let (Some(first), Some(last)) = (res.first_usable_host, res.last_usable_host) {
        if first == last {
            print_row("Usable Host", &first.to_string().bright_cyan());
        } else {
            print_row(
                "Usable Host Range",
                &format!(
                    "{} - {}",
                    first.to_string().bright_cyan(),
                    last.to_string().bright_cyan()
                ),
            );
        }
    } else {
        print_row("Usable Host Range", "None");
    }

    let usable_str = format_u64_commas(res.usable_hosts);
    let total_str = format_u64_commas(res.total_addresses);
    print_row("Usable Hosts", &usable_str.bright_green());
    print_row("Total Addresses", &total_str);
    if let Some(ref note) = res.host_range_note {
        print_row("Host Note", &note.bright_magenta());
    }

    print_header("Classification");
    print_row("Address Class", &res.ip_class);
    print_row("Scope / Type", &res.scope.bright_yellow());

    print_header("Representations");
    print_row("Integer (Decimal)", &res.integer_id.to_string());
    print_row("Hexadecimal", &res.ip_hex);
    print_row("IP Binary", &res.ip_binary);
    print_row("Netmask Binary", &res.netmask_binary);
    print_row("Wildcard Binary", &res.wildcard_binary);

    if let Some(ref subs) = res.subnets {
        let total_count = res.total_subnets_count.unwrap_or(subs.len() as u64);
        print_header(&format!(
            "Subnet Division (Total: {} subnets)",
            format_u64_commas(total_count)
        ));

        println!(
            "  {:<4} {:<18} {:<16} {:<33} {:<16} {:<10}",
            "#".bright_black(),
            "CIDR".cyan().bold(),
            "Network".bright_black(),
            "Usable Range".bright_black(),
            "Broadcast".bright_black(),
            "Hosts".bright_black()
        );
        println!("  {}", "-".repeat(102).bright_black());

        for s in subs {
            let range_str = match (s.first_usable_host, s.last_usable_host) {
                (Some(f), Some(l)) if f == l => f.to_string(),
                (Some(f), Some(l)) => format!("{f} - {l}"),
                _ => "N/A".to_string(),
            };
            println!(
                "  {:<4} {:<18} {:<16} {:<33} {:<16} {:<10}",
                s.subnet_index,
                s.cidr.green().bold(),
                s.network_address,
                range_str,
                s.broadcast_address,
                format_u64_commas(s.usable_hosts)
            );
        }

        if total_count > subs.len() as u64 {
            println!(
                "  {}",
                format!(
                    "... and {} more subnets omitted from display.",
                    total_count - subs.len() as u64
                )
                .italic()
                .bright_black()
            );
        }
    }
    println!();
}

fn print_ipv6(res: &Ipv6CalculationResult) {
    print_header("IPv6 Subnet Calculation");
    print_row("Input", &res.input);
    print_row(
        "CIDR Prefix",
        &format!("/{}", res.prefix_len).bright_yellow(),
    );
    print_row("Network Address", &res.network_address.to_string().green());
    print_row("Compressed Format", &res.compressed.green());
    print_row("Expanded Format", &res.expanded);
    print_row("Subnet Netmask", &res.netmask);

    print_header("Capacity & Address Range");
    print_row(
        "First Address",
        &res.first_address.to_string().bright_cyan(),
    );
    print_row("Last Address", &res.last_address.to_string().bright_cyan());
    print_row(
        "Address Range",
        &format!(
            "{} -\n                         {}",
            res.first_address, res.last_address
        ),
    );
    print_row(
        "Total Addresses",
        &res.total_addresses_formatted.bright_green(),
    );
    if let Some(ref note) = res.host_range_note {
        print_row("Scope Note", &note.bright_magenta());
    }

    print_header("Classification");
    print_row("Scope / Type", &res.scope.bright_yellow());

    if let Some(ref subs) = res.subnets {
        let count_display = res.total_subnets_count.as_deref().unwrap_or("N/A");
        print_header(&format!("Subnet Division (Total: {count_display} subnets)"));

        println!(
            "  {:<4} {:<32} {:<38} {:<38}",
            "#".bright_black(),
            "CIDR".cyan().bold(),
            "First Address".bright_black(),
            "Last Address".bright_black()
        );
        println!("  {}", "-".repeat(116).bright_black());

        for s in subs {
            println!(
                "  {:<4} {:<32} {:<38} {:<38}",
                s.subnet_index,
                s.cidr.green().bold(),
                s.first_address,
                s.last_address
            );
        }

        if let Some(ref total_s) = res.total_subnets_count {
            if subs.len() == 256 {
                println!(
                    "  {}",
                    format!("... displaying first 256 of {total_s} subnets.")
                        .italic()
                        .bright_black()
                );
            }
        }
    }
    println!();
}

fn format_u64_commas(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let len = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(c);
    }
    result
}
