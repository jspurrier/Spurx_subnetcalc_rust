use crate::types::{Ipv4CalculationResult, Ipv4SubnetInfo};
use ipnet::Ipv4Net;
use std::net::Ipv4Addr;

pub fn calculate_ipv4(
    net: &Ipv4Net,
    raw_input: &str,
    split_prefix: Option<u8>,
) -> Result<Ipv4CalculationResult, String> {
    let input_ip = net.addr();
    let network_address = net.network();
    let broadcast_address = net.broadcast();
    let netmask = net.netmask();
    let wildcard_mask = net.hostmask();
    let prefix_len = net.prefix_len();

    let host_bits = 32 - prefix_len;
    let total_addresses: u64 = if prefix_len == 0 {
        1u64 << 32
    } else {
        1u64 << host_bits
    };

    let (usable_hosts, first_usable_host, last_usable_host, host_range_note) = match prefix_len {
        32 => (
            1,
            Some(network_address),
            Some(network_address),
            Some("Single Host (/32)".to_string()),
        ),
        31 => (
            2,
            Some(network_address),
            Some(broadcast_address),
            Some(
                "RFC 3021 Point-to-Point Link (2 hosts, no dedicated network/broadcast)"
                    .to_string(),
            ),
        ),
        _ => {
            let net_bits = u32::from(network_address);
            let bcast_bits = u32::from(broadcast_address);
            let first = Ipv4Addr::from(net_bits + 1);
            let last = Ipv4Addr::from(bcast_bits - 1);
            (
                total_addresses.saturating_sub(2),
                Some(first),
                Some(last),
                None,
            )
        }
    };

    let ip_class = classify_ipv4_class(network_address);
    let scope = classify_ipv4_scope(network_address);

    let ip_binary = format_ipv4_binary(network_address);
    let netmask_binary = format_ipv4_binary(netmask);
    let wildcard_binary = format_ipv4_binary(wildcard_mask);

    let integer_id = u32::from(network_address);
    let ip_hex = format!("0x{:08X}", integer_id);

    let (subnets, total_subnets_count) = if let Some(new_prefix) = split_prefix {
        if new_prefix <= prefix_len {
            return Err(format!(
                "Split prefix /{new_prefix} must be greater than current prefix /{prefix_len}"
            ));
        }
        if new_prefix > 32 {
            return Err("Split prefix cannot exceed /32".to_string());
        }

        let diff = new_prefix - prefix_len;
        let total_subs = 1u64 << diff;
        let subnets_iter = net
            .subnets(new_prefix)
            .map_err(|e| format!("Failed to generate subnets: {e}"))?;

        // Limit stored subnets to first 256 for display memory efficiency
        let mut subs = Vec::new();
        for (i, sub) in subnets_iter.enumerate() {
            if i >= 256 {
                break;
            }
            let (sub_usable, sub_first, sub_last) = match sub.prefix_len() {
                32 => (1, Some(sub.network()), Some(sub.network())),
                31 => (2, Some(sub.network()), Some(sub.broadcast())),
                _ => {
                    let f = Ipv4Addr::from(u32::from(sub.network()) + 1);
                    let l = Ipv4Addr::from(u32::from(sub.broadcast()) - 1);
                    let total = 1u64 << (32 - sub.prefix_len());
                    (total.saturating_sub(2), Some(f), Some(l))
                }
            };
            subs.push(Ipv4SubnetInfo {
                subnet_index: i + 1,
                cidr: sub.to_string(),
                network_address: sub.network(),
                broadcast_address: sub.broadcast(),
                first_usable_host: sub_first,
                last_usable_host: sub_last,
                usable_hosts: sub_usable,
            });
        }
        (Some(subs), Some(total_subs))
    } else {
        (None, None)
    };

    Ok(Ipv4CalculationResult {
        input: raw_input.to_string(),
        input_ip,
        network_address,
        broadcast_address,
        netmask,
        wildcard_mask,
        prefix_len,
        total_addresses,
        usable_hosts,
        first_usable_host,
        last_usable_host,
        host_range_note,
        ip_class,
        scope,
        ip_binary,
        netmask_binary,
        wildcard_binary,
        ip_hex,
        integer_id,
        subnets,
        total_subnets_count,
    })
}

pub fn format_ipv4_binary(ip: Ipv4Addr) -> String {
    let octets = ip.octets();
    format!(
        "{:08b}.{:08b}.{:08b}.{:08b}",
        octets[0], octets[1], octets[2], octets[3]
    )
}

fn classify_ipv4_class(ip: Ipv4Addr) -> String {
    let first = ip.octets()[0];
    match first {
        0..=127 => "Class A (Default mask /8)".to_string(),
        128..=191 => "Class B (Default mask /16)".to_string(),
        192..=223 => "Class C (Default mask /24)".to_string(),
        224..=239 => "Class D (Multicast)".to_string(),
        _ => "Class E (Experimental / Reserved)".to_string(),
    }
}

fn classify_ipv4_scope(ip: Ipv4Addr) -> String {
    let octets = ip.octets();
    let o0 = octets[0];
    let o1 = octets[1];

    if o0 == 10 {
        return "Private Network (RFC 1918 10.0.0.0/8)".to_string();
    }
    if o0 == 172 && (16..=31).contains(&o1) {
        return "Private Network (RFC 1918 172.16.0.0/12)".to_string();
    }
    if o0 == 192 && o1 == 168 {
        return "Private Network (RFC 1918 192.168.0.0/16)".to_string();
    }
    if o0 == 127 {
        return "Loopback (RFC 1122 127.0.0.0/8)".to_string();
    }
    if o0 == 169 && o1 == 254 {
        return "Link-Local / APIPA (RFC 3927 169.254.0.0/16)".to_string();
    }
    if o0 == 100 && (64..=127).contains(&o1) {
        return "Carrier-Grade NAT / CGNAT (RFC 6598 100.64.0.0/10)".to_string();
    }
    if (224..=239).contains(&o0) {
        return "Multicast (RFC 5771 224.0.0.0/4)".to_string();
    }
    if o0 == 192 && o1 == 0 && octets[2] == 2 {
        return "Documentation TEST-NET-1 (RFC 5737 192.0.2.0/24)".to_string();
    }
    if o0 == 198 && o1 == 51 && octets[2] == 100 {
        return "Documentation TEST-NET-2 (RFC 5737 198.51.100.0/24)".to_string();
    }
    if o0 == 203 && o1 == 0 && octets[2] == 113 {
        return "Documentation TEST-NET-3 (RFC 5737 203.0.113.0/24)".to_string();
    }
    if o0 == 198 && (o1 == 18 || o1 == 19) {
        return "Benchmark Testing (RFC 2544 198.18.0.0/15)".to_string();
    }
    if o0 == 0 {
        return "Current Network / This Host (RFC 1122 0.0.0.0/8)".to_string();
    }
    if ip == Ipv4Addr::BROADCAST {
        return "Limited Broadcast (RFC 919 255.255.255.255/32)".to_string();
    }

    "Public Internet Unicast".to_string()
}
