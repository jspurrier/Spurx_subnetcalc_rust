use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CalculationResult {
    #[serde(rename = "ipv4")]
    V4(Ipv4CalculationResult),
    #[serde(rename = "ipv6")]
    V6(Ipv6CalculationResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ipv4CalculationResult {
    pub input: String,
    pub input_ip: Ipv4Addr,
    pub network_address: Ipv4Addr,
    pub broadcast_address: Ipv4Addr,
    pub netmask: Ipv4Addr,
    pub wildcard_mask: Ipv4Addr,
    pub prefix_len: u8,
    pub total_addresses: u64,
    pub usable_hosts: u64,
    pub first_usable_host: Option<Ipv4Addr>,
    pub last_usable_host: Option<Ipv4Addr>,
    pub host_range_note: Option<String>,
    pub ip_class: String,
    pub scope: String,
    pub ip_binary: String,
    pub netmask_binary: String,
    pub wildcard_binary: String,
    pub ip_hex: String,
    pub integer_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<Ipv4SubnetInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_subnets_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ipv4SubnetInfo {
    pub subnet_index: usize,
    pub cidr: String,
    pub network_address: Ipv4Addr,
    pub broadcast_address: Ipv4Addr,
    pub first_usable_host: Option<Ipv4Addr>,
    pub last_usable_host: Option<Ipv4Addr>,
    pub usable_hosts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ipv6CalculationResult {
    pub input: String,
    pub input_ip: Ipv6Addr,
    pub network_address: Ipv6Addr,
    pub compressed: String,
    pub expanded: String,
    pub netmask: String,
    pub prefix_len: u8,
    pub first_address: Ipv6Addr,
    pub last_address: Ipv6Addr,
    pub total_addresses_exponent: u8,
    pub total_addresses_formatted: String,
    pub scope: String,
    pub host_range_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<Ipv6SubnetInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_subnets_count: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ipv6SubnetInfo {
    pub subnet_index: usize,
    pub cidr: String,
    pub network_address: Ipv6Addr,
    pub first_address: Ipv6Addr,
    pub last_address: Ipv6Addr,
}
