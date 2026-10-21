use crate::types::{Ipv6CalculationResult, Ipv6SubnetInfo};
use ipnet::Ipv6Net;
use std::net::Ipv6Addr;

pub fn calculate_ipv6(
    net: &Ipv6Net,
    raw_input: &str,
    split_prefix: Option<u8>,
) -> Result<Ipv6CalculationResult, String> {
    let input_ip = net.addr();
    let network_address = net.network();
    let netmask = net.netmask();
    let hostmask = net.hostmask();
    let prefix_len = net.prefix_len();

    let compressed = network_address.to_string();
    let expanded = format_ipv6_expanded(network_address);

    let net_u128 = u128::from(network_address);
    let hostmask_u128 = u128::from(hostmask);
    let last_address_u128 = net_u128 | hostmask_u128;
    let first_address = network_address;
    let last_address = Ipv6Addr::from(last_address_u128);

    let host_bits = 128 - prefix_len;
    let total_addresses_exponent = host_bits;
    let total_addresses_formatted = format_ipv6_total_addresses(host_bits);

    let scope = classify_ipv6_scope(network_address);

    let host_range_note = match prefix_len {
        128 => Some("Single Host (/128)".to_string()),
        127 => Some("RFC 6164 Point-to-Point Inter-Router Link (2 addresses)".to_string()),
        64 => Some("Standard IPv6 Subnet (SLAAC / Interface ID: 64-bit)".to_string()),
        _ => None,
    };

    let (subnets, total_subnets_count) = if let Some(new_prefix) = split_prefix {
        if new_prefix <= prefix_len {
            return Err(format!(
                "Split prefix /{new_prefix} must be greater than current prefix /{prefix_len}"
            ));
        }
        if new_prefix > 128 {
            return Err("Split prefix cannot exceed /128".to_string());
        }

        let diff = new_prefix - prefix_len;
        let total_subnets_str = if diff <= 64 {
            format!("{}", 1u128 << diff)
        } else {
            format!("2^{diff} (~{:.4e})", 2.0f64.powi(diff as i32))
        };

        let subnets_iter = net
            .subnets(new_prefix)
            .map_err(|e| format!("Failed to generate subnets: {e}"))?;

        let mut subs = Vec::new();
        for (i, sub) in subnets_iter.enumerate() {
            if i >= 256 {
                break;
            }
            let sub_first = sub.network();
            let sub_last = Ipv6Addr::from(u128::from(sub.network()) | u128::from(sub.hostmask()));
            subs.push(Ipv6SubnetInfo {
                subnet_index: i + 1,
                cidr: sub.to_string(),
                network_address: sub.network(),
                first_address: sub_first,
                last_address: sub_last,
            });
        }
        (Some(subs), Some(total_subnets_str))
    } else {
        (None, None)
    };

    Ok(Ipv6CalculationResult {
        input: raw_input.to_string(),
        input_ip,
        network_address,
        compressed,
        expanded,
        netmask: netmask.to_string(),
        prefix_len,
        first_address,
        last_address,
        total_addresses_exponent,
        total_addresses_formatted,
        scope,
        host_range_note,
        subnets,
        total_subnets_count,
    })
}

pub fn format_ipv6_expanded(ip: Ipv6Addr) -> String {
    let seg = ip.segments();
    format!(
        "{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}",
        seg[0], seg[1], seg[2], seg[3], seg[4], seg[5], seg[6], seg[7]
    )
}

fn format_ipv6_total_addresses(host_bits: u8) -> String {
    if host_bits == 0 {
        return "1 (2^0)".to_string();
    }
    if host_bits <= 64 {
        let val = 1u128 << host_bits;
        let formatted = format_number_with_commas(val);
        format!("{formatted} (2^{host_bits})")
    } else if host_bits <= 127 {
        let val = 1u128 << host_bits;
        let sci = 2.0f64.powi(host_bits as i32);
        let formatted = format_number_with_commas(val);
        format!("{formatted} (2^{host_bits} ≈ {sci:.4e})")
    } else {
        // host_bits == 128
        "340,282,366,920,938,463,463,374,607,431,768,211,456 (2^128 ≈ 3.4028e38)".to_string()
    }
}

fn format_number_with_commas(n: u128) -> String {
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

fn classify_ipv6_scope(ip: Ipv6Addr) -> String {
    let seg = ip.segments();

    if ip == Ipv6Addr::LOCALHOST {
        return "Loopback (RFC 4291 ::1/128)".to_string();
    }
    if ip == Ipv6Addr::UNSPECIFIED {
        return "Unspecified (RFC 4291 ::/128)".to_string();
    }
    // IPv4-mapped (::ffff:0:0/96)
    if seg[0] == 0 && seg[1] == 0 && seg[2] == 0 && seg[3] == 0 && seg[4] == 0 && seg[5] == 0xffff {
        return "IPv4-Mapped Address (RFC 4291 ::ffff:0:0/96)".to_string();
    }
    // IPv4-compatible (::/96)
    if seg[0] == 0
        && seg[1] == 0
        && seg[2] == 0
        && seg[3] == 0
        && seg[4] == 0
        && seg[5] == 0
        && (seg[6] != 0 || seg[7] != 0)
    {
        return "IPv4-Compatible Address (RFC 4291 deprecated ::/96)".to_string();
    }
    // Discard-Only (100::/64)
    if seg[0] == 0x0100 && seg[1] == 0 && seg[2] == 0 && seg[3] == 0 {
        return "Discard-Only (RFC 6666 100::/64)".to_string();
    }
    // Documentation (2001:db8::/32)
    if seg[0] == 0x2001 && seg[1] == 0x0db8 {
        return "Documentation (RFC 3849 2001:db8::/32)".to_string();
    }
    // Teredo (2001::/32)
    if seg[0] == 0x2001 && seg[1] == 0 {
        return "Teredo Tunneling (RFC 4380 2001::/32)".to_string();
    }
    // Benchmarking (2001:2::/48)
    if seg[0] == 0x2001 && seg[1] == 0x0002 {
        return "Benchmarking (RFC 5180 2001:2::/48)".to_string();
    }
    // ORCHIDv2 (2001:20::/28)
    if seg[0] == 0x2001 && (seg[1] & 0xfff0) == 0x0020 {
        return "ORCHIDv2 (RFC 7343 2001:20::/28)".to_string();
    }
    // 6to4 (2002::/16)
    if seg[0] == 0x2002 {
        return "6to4 Transition (RFC 3056 2002::/16)".to_string();
    }
    // Unique Local (fc00::/7) -> first byte is 0xfc or 0xfd
    if (seg[0] & 0xfe00) == 0xfc00 {
        return "Unique Local Unicast / ULA (RFC 4193 fc00::/7)".to_string();
    }
    // Link-Local (fe80::/10) -> first 10 bits are 1111 1110 10 -> 0xfe80..0xfebf
    if (seg[0] & 0xffc0) == 0xfe80 {
        return "Link-Local Unicast (RFC 4291 fe80::/10)".to_string();
    }
    // Multicast (ff00::/8) -> first 8 bits are 1111 1111 -> 0xff..
    if (seg[0] & 0xff00) == 0xff00 {
        return "Multicast (RFC 4291 ff00::/8)".to_string();
    }
    // Global Unicast (2000::/3) -> first 3 bits are 001 -> 0x2000..0x3fff
    if (seg[0] & 0xe000) == 0x2000 {
        return "Global Unicast (RFC 3587 2000::/3)".to_string();
    }

    "Reserved / Unassigned".to_string()
}
