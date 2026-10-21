use spurx_ipcalc::calculate;
use spurx_ipcalc::types::CalculationResult;
use std::net::{Ipv4Addr, Ipv6Addr};

#[test]
fn test_ipv4_standard_subnet() {
    let result = calculate("192.168.1.50/24", None, None).expect("Calculation should succeed");
    if let CalculationResult::V4(v4) = result {
        assert_eq!(v4.network_address, Ipv4Addr::new(192, 168, 1, 0));
        assert_eq!(v4.broadcast_address, Ipv4Addr::new(192, 168, 1, 255));
        assert_eq!(v4.netmask, Ipv4Addr::new(255, 255, 255, 0));
        assert_eq!(v4.wildcard_mask, Ipv4Addr::new(0, 0, 0, 255));
        assert_eq!(v4.prefix_len, 24);
        assert_eq!(v4.total_addresses, 256);
        assert_eq!(v4.usable_hosts, 254);
        assert_eq!(v4.first_usable_host, Some(Ipv4Addr::new(192, 168, 1, 1)));
        assert_eq!(v4.last_usable_host, Some(Ipv4Addr::new(192, 168, 1, 254)));
        assert_eq!(v4.ip_class, "Class C (Default mask /24)");
        assert!(v4.scope.contains("RFC 1918"));
    } else {
        panic!("Expected IPv4 calculation result");
    }
}

#[test]
fn test_ipv4_slash_31_point_to_point() {
    let result = calculate("10.0.0.0/31", None, None).expect("Calculation should succeed");
    if let CalculationResult::V4(v4) = result {
        assert_eq!(v4.total_addresses, 2);
        assert_eq!(v4.usable_hosts, 2);
        assert_eq!(v4.first_usable_host, Some(Ipv4Addr::new(10, 0, 0, 0)));
        assert_eq!(v4.last_usable_host, Some(Ipv4Addr::new(10, 0, 0, 1)));
        assert!(v4.host_range_note.unwrap().contains("RFC 3021"));
    } else {
        panic!("Expected IPv4 calculation result");
    }
}

#[test]
fn test_ipv4_slash_32_single_host() {
    let result = calculate("1.1.1.1/32", None, None).expect("Calculation should succeed");
    if let CalculationResult::V4(v4) = result {
        assert_eq!(v4.total_addresses, 1);
        assert_eq!(v4.usable_hosts, 1);
        assert_eq!(v4.first_usable_host, Some(Ipv4Addr::new(1, 1, 1, 1)));
        assert_eq!(v4.last_usable_host, Some(Ipv4Addr::new(1, 1, 1, 1)));
        assert!(v4.scope.contains("Public"));
    } else {
        panic!("Expected IPv4 calculation result");
    }
}

#[test]
fn test_ipv4_with_netmask_arg() {
    let result =
        calculate("172.16.0.0", Some("255.255.0.0"), None).expect("Calculation should succeed");
    if let CalculationResult::V4(v4) = result {
        assert_eq!(v4.prefix_len, 16);
        assert_eq!(v4.total_addresses, 65536);
        assert_eq!(v4.usable_hosts, 65534);
        assert!(v4.scope.contains("172.16.0.0/12"));
    } else {
        panic!("Expected IPv4 calculation result");
    }
}

#[test]
fn test_ipv4_subnet_splitting() {
    let result = calculate("192.168.0.0/24", None, Some(26)).expect("Calculation should succeed");
    if let CalculationResult::V4(v4) = result {
        assert_eq!(v4.total_subnets_count, Some(4));
        let subs = v4.subnets.expect("Should contain subnets list");
        assert_eq!(subs.len(), 4);
        assert_eq!(subs[0].cidr, "192.168.0.0/26");
        assert_eq!(subs[0].network_address, Ipv4Addr::new(192, 168, 0, 0));
        assert_eq!(
            subs[0].first_usable_host,
            Some(Ipv4Addr::new(192, 168, 0, 1))
        );
        assert_eq!(
            subs[0].last_usable_host,
            Some(Ipv4Addr::new(192, 168, 0, 62))
        );
        assert_eq!(subs[0].broadcast_address, Ipv4Addr::new(192, 168, 0, 63));
        assert_eq!(subs[0].usable_hosts, 62);

        assert_eq!(subs[3].cidr, "192.168.0.192/26");
        assert_eq!(subs[3].network_address, Ipv4Addr::new(192, 168, 0, 192));
        assert_eq!(
            subs[3].first_usable_host,
            Some(Ipv4Addr::new(192, 168, 0, 193))
        );
        assert_eq!(
            subs[3].last_usable_host,
            Some(Ipv4Addr::new(192, 168, 0, 254))
        );
        assert_eq!(subs[3].broadcast_address, Ipv4Addr::new(192, 168, 0, 255));
    } else {
        panic!("Expected IPv4 calculation result");
    }
}

#[test]
fn test_ipv6_standard_subnet() {
    let result =
        calculate("2001:db8:abcd:0012::/64", None, None).expect("Calculation should succeed");
    if let CalculationResult::V6(v6) = result {
        assert_eq!(v6.prefix_len, 64);
        assert_eq!(
            v6.network_address,
            "2001:db8:abcd:12::".parse::<Ipv6Addr>().unwrap()
        );
        assert_eq!(v6.expanded, "2001:0db8:abcd:0012:0000:0000:0000:0000");
        assert_eq!(
            v6.first_address,
            "2001:db8:abcd:12::".parse::<Ipv6Addr>().unwrap()
        );
        assert_eq!(
            v6.last_address,
            "2001:db8:abcd:12:ffff:ffff:ffff:ffff"
                .parse::<Ipv6Addr>()
                .unwrap()
        );
        assert!(v6.scope.contains("RFC 3849"));
    } else {
        panic!("Expected IPv6 calculation result");
    }
}

#[test]
fn test_ipv6_ula_and_link_local() {
    let ula_res = calculate("fd12:3456:789a::/48", None, None).unwrap();
    if let CalculationResult::V6(v6) = ula_res {
        assert!(v6.scope.contains("Unique Local"));
    } else {
        panic!("Expected IPv6");
    }

    let ll_res = calculate("fe80::1/64", None, None).unwrap();
    if let CalculationResult::V6(v6) = ll_res {
        assert!(v6.scope.contains("Link-Local"));
    } else {
        panic!("Expected IPv6");
    }
}

#[test]
fn test_ipv6_subnet_splitting() {
    let result = calculate("2001:db8::/48", None, Some(50)).expect("Calculation should succeed");
    if let CalculationResult::V6(v6) = result {
        assert_eq!(v6.total_subnets_count, Some("4".to_string()));
        let subs = v6.subnets.expect("Should contain subnets list");
        assert_eq!(subs.len(), 4);
        assert_eq!(subs[0].cidr, "2001:db8::/50");
        assert_eq!(subs[1].cidr, "2001:db8:0:4000::/50");
        assert_eq!(subs[2].cidr, "2001:db8:0:8000::/50");
        assert_eq!(subs[3].cidr, "2001:db8:0:c000::/50");
    } else {
        panic!("Expected IPv6 calculation result");
    }
}

#[test]
fn test_json_serialization() {
    let result = calculate("192.168.0.0/24", None, None).unwrap();
    let json_str = serde_json::to_string(&result).expect("Serialization failed");
    assert!(json_str.contains("\"network_address\":\"192.168.0.0\""));
    assert!(json_str.contains("\"usable_hosts\":254"));
}
