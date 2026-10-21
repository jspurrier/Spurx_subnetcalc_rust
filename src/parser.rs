use ipnet::{IpNet, Ipv4Net, Ipv6Net};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

/// Parses IP network input from either:
/// 1. CIDR notation: "192.168.0.0/24", "2001:db8::/32"
/// 2. IP address and separate netmask: "192.168.0.0", Some("255.255.255.0")
/// 3. Single host IP without mask: "192.168.0.1" (defaults to /32 or /128)
pub fn parse_network_input(input: &str, netmask: Option<&str>) -> Result<IpNet, String> {
    let trimmed = input.trim();

    if let Some(mask_str) = netmask {
        let mask_trimmed = mask_str.trim();
        // Check if IPv4 with netmask
        if let (Ok(ip), Ok(mask)) = (
            trimmed.parse::<Ipv4Addr>(),
            mask_trimmed.parse::<Ipv4Addr>(),
        ) {
            return Ipv4Net::with_netmask(ip, mask)
                .map(IpNet::V4)
                .map_err(|e| format!("Invalid IPv4 netmask '{mask_str}': {e}"));
        }
        // Check if IPv6 with netmask
        if let (Ok(ip), Ok(mask)) = (
            trimmed.parse::<Ipv6Addr>(),
            mask_trimmed.parse::<Ipv6Addr>(),
        ) {
            return Ipv6Net::with_netmask(ip, mask)
                .map(IpNet::V6)
                .map_err(|e| format!("Invalid IPv6 netmask '{mask_str}': {e}"));
        }
        // Check if mask_str is just a prefix length (e.g. "24" or "/24")
        let clean_mask = mask_trimmed.trim_start_matches('/');
        if let Ok(prefix) = clean_mask.parse::<u8>() {
            let combined = format!("{trimmed}/{prefix}");
            return IpNet::from_str(&combined)
                .map_err(|e| format!("Invalid network '{combined}': {e}"));
        }
        return Err(format!(
            "Could not parse IP '{trimmed}' with netmask '{mask_trimmed}'"
        ));
    }

    // Direct CIDR parse
    if trimmed.contains('/') {
        return IpNet::from_str(trimmed)
            .map_err(|e| format!("Invalid CIDR prefix '{trimmed}': {e}"));
    }

    // Single IPv4 without prefix -> default to /32
    if let Ok(ipv4) = trimmed.parse::<Ipv4Addr>() {
        return Ipv4Net::new(ipv4, 32)
            .map(IpNet::V4)
            .map_err(|e| format!("Failed to create IPv4 /32 for '{trimmed}': {e}"));
    }

    // Single IPv6 without prefix -> default to /128
    if let Ok(ipv6) = trimmed.parse::<Ipv6Addr>() {
        return Ipv6Net::new(ipv6, 128)
            .map(IpNet::V6)
            .map_err(|e| format!("Failed to create IPv6 /128 for '{trimmed}': {e}"));
    }

    Err(format!(
        "Invalid IP or CIDR address '{trimmed}'. Example valid inputs: 192.168.0.0/24, 10.0.0.1, 2001:db8::/32"
    ))
}
