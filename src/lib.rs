pub mod display;
pub mod ipv4;
pub mod ipv6;
pub mod parser;
pub mod types;

use ipnet::IpNet;
use types::CalculationResult;

/// Perform subnet calculation given an input string and optional netmask and split prefix
pub fn calculate(
    input: &str,
    netmask: Option<&str>,
    split_prefix: Option<u8>,
) -> Result<CalculationResult, String> {
    let parsed_net = parser::parse_network_input(input, netmask)?;
    match parsed_net {
        IpNet::V4(v4_net) => {
            let res = ipv4::calculate_ipv4(&v4_net, input, split_prefix)?;
            Ok(CalculationResult::V4(res))
        }
        IpNet::V6(v6_net) => {
            let res = ipv6::calculate_ipv6(&v6_net, input, split_prefix)?;
            Ok(CalculationResult::V6(res))
        }
    }
}
