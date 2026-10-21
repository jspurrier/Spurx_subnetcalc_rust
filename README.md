# Spurx IP Calculator (`spurx-ipcalc`)

A fast, cross-platform Command Line Interface (CLI) subnet calculator written in Rust using the [`ipnet`](https://crates.io/crates/ipnet) crate. It computes network parameters, address ranges, usable hosts, binary/hex representations, address scopes, and subnet divisions for both IPv4 and IPv6 networks.

Supports **Linux**, **macOS**, and **Windows**.

---

## Features

- **IPv4 Subnet Calculations**:
  - Network address, broadcast address, netmask, and wildcard mask.
  - Usable host range and host counts.
  - Handles edge-case subnets:
    - Standard subnets (`/1` - `/30`): standard usable host calculation ($2^{\text{host bits}} - 2$).
    - Point-to-point links (`/31`): [RFC 3021](https://datatracker.ietf.org/doc/html/rfc3021) 2-host support without dedicated network/broadcast.
    - Single host (`/32`): 1 host address.
  - Legacy address class detection (Class A, B, C, D Multicast, E Experimental).
  - Scope and RFC classification (RFC 1918 Private, RFC 1122 Loopback, RFC 3927 Link-Local, RFC 6598 CGNAT, RFC 5771 Multicast, RFC 5737 TEST-NET, Public Internet Unicast).
  - Decimal, hexadecimal, and 32-bit dotted binary representations for IP, netmask, and wildcard mask.

- **IPv6 Subnet Calculations**:
  - Network address, prefix length, and subnet netmask.
  - Compressed ([RFC 5952](https://datatracker.ietf.org/doc/html/rfc5952)) and full 8-group expanded address formats.
  - Address range (first and last address).
  - Address count (exact and $2^N$ / scientific notation for massive address spaces).
  - Scope and type classification (Global Unicast, Unique Local / ULA [RFC 4193], Link-Local [RFC 4291], Multicast, Loopback, Documentation [RFC 3849], etc.).

- **Subnet Splitting (`--split <prefix>`)**:
  - Divide any IPv4 or IPv6 network into smaller subnets (e.g. split `/24` into `/26`s or `/48` into `/64`s).
  - Displays subnet CIDRs, network addresses, usable ranges, broadcast addresses, and host counts.

- **Output Options**:
  - **Colorized Terminal Table**: Clean, aligned sections with color-coded highlights.
  - **JSON Mode (`--json`)**: Structured output suitable for scripts, automation, and CI/CD.
  - **No Color (`--no-color`)**: Complies with standard no-color workflows and terminal redirects.
  - **Interactive Mode**: Automatically prompts if executed without arguments.

- **Maximum Binary & Performance Optimization**:
  - Compiled with Fat Link-Time Optimization (`lto = "fat"`).
  - Single codegen unit (`codegen-units = 1`) for inter-crate inlining and dead code elimination.
  - Level 3 optimizations (`opt-level = 3`).
  - Abort on panic (`panic = "abort"`) to strip unwinding tables.
  - Symbol stripping (`strip = true`) for minimal binary footprint and lightning-fast start time.

---

## Installation & Building

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (version 1.70 or newer).

### Build from Source
```bash
git clone https://github.com/spurx/Spurx_subnet_calc_rust.git
cd Spurx_subnet_calc_rust

# Build release binary (fully optimized)
cargo build --release

# The compiled binary is located at target/release/spurx-ipcalc
./target/release/spurx-ipcalc --version
```

### Install Globally
```bash
cargo install --path .
```
Now `spurx-ipcalc` is available in your PATH.

---

## Cross-Platform Builds

This project has zero platform-dependent C libraries or Unix syscalls. You can compile it for Linux, macOS, and Windows.

### Cross-compiling with `cross` or `cargo`:
- **Windows (from Linux/macOS)**:
  ```bash
  rustup target add x86_64-pc-windows-gnu
  cargo build --release --target x86_64-pc-windows-gnu
  # Outputs target/x86_64-pc-windows-gnu/release/spurx-ipcalc.exe
  ```
- **Linux (musl static binary)**:
  ```bash
  rustup target add x86_64-unknown-linux-musl
  cargo build --release --target x86_64-unknown-linux-musl
  ```
- **macOS (Apple Silicon & Intel)**:
  ```bash
  rustup target add aarch64-apple-darwin
  rustup target add x86_64-apple-darwin
  cargo build --release --target aarch64-apple-darwin
  ```

---

## Usage Examples

### 1. Basic IPv4 Subnet Calculation
```bash
spurx-ipcalc 192.168.0.0/24
```
**Output:**
```text
=== IPv4 Subnet Calculation ===
  Input                  : 192.168.0.0/24
  CIDR Prefix            : /24
  Network Address        : 192.168.0.0
  Broadcast Address      : 192.168.0.255
  Subnet Netmask         : 255.255.255.0
  Wildcard (Hostmask)    : 0.0.0.255

=== Capacity & Host Range ===
  Usable Host Range      : 192.168.0.1 - 192.168.0.254
  Usable Hosts           : 254
  Total Addresses        : 256

=== Classification ===
  Address Class          : Class C (Default mask /24)
  Scope / Type           : Private Network (RFC 1918 192.168.0.0/16)

=== Representations ===
  Integer (Decimal)      : 3232235520
  Hexadecimal            : 0xC0A80000
  IP Binary              : 11000000.10101000.00000000.00000000
  Netmask Binary         : 11111111.11111111.11111111.00000000
  Wildcard Binary        : 00000000.00000000.00000000.11111111
```

### 2. Host IP with Subnet Mask
You can pass any host IP inside a subnet, or specify the netmask separately:
```bash
spurx-ipcalc 10.50.23.111/19
# Or separate netmask:
spurx-ipcalc 10.0.0.0 255.255.240.0
```

### 3. Point-to-Point `/31` Links (RFC 3021)
```bash
spurx-ipcalc 172.16.10.4/31
```
Output correctly recognizes 2 usable hosts with no wasted network/broadcast addresses.

### 4. IPv6 Subnet Calculation
```bash
spurx-ipcalc 2001:db8:85a3::/64
```
**Output:**
```text
=== IPv6 Subnet Calculation ===
  Input                  : 2001:db8:85a3::/64
  CIDR Prefix            : /64
  Network Address        : 2001:db8:85a3::
  Compressed Format      : 2001:db8:85a3::
  Expanded Format        : 2001:0db8:85a3:0000:0000:0000:0000:0000
  Subnet Netmask         : ffff:ffff:ffff:ffff::

=== Capacity & Address Range ===
  First Address          : 2001:db8:85a3::
  Last Address           : 2001:db8:85a3:0:ffff:ffff:ffff:ffff
  Address Range          : 2001:db8:85a3:: -
                         2001:db8:85a3:0:ffff:ffff:ffff:ffff
  Total Addresses        : 18,446,744,073,709,551,616 (2^64)
  Scope Note             : Standard IPv6 Subnet (SLAAC / Interface ID: 64-bit)

=== Classification ===
  Scope / Type           : Documentation (RFC 3849 2001:db8::/32)
```

### 5. Subnet Splitting (`--split`)
Divide a network into smaller child subnets:
```bash
spurx-ipcalc 192.168.0.0/24 --split 26
```
**Output:**
```text
=== Subnet Division (Total: 4 subnets) ===
  #    CIDR               Network          Usable Range                      Broadcast        Hosts     
  ------------------------------------------------------------------------------------------------------
  1    192.168.0.0/26     192.168.0.0      192.168.0.1 - 192.168.0.62        192.168.0.63     62        
  2    192.168.0.64/26    192.168.0.64     192.168.0.65 - 192.168.0.126      192.168.0.127    62        
  3    192.168.0.128/26   192.168.0.128    192.168.0.129 - 192.168.0.190     192.168.0.191    62        
  4    192.168.0.192/26   192.168.0.192    192.168.0.193 - 192.168.0.254     192.168.0.255    62        
```

### 6. Automation & Scripting with JSON (`--json`)
```bash
spurx-ipcalc 192.168.0.0/24 --json | jq .usable_hosts
# Output: 254
```

### 7. Interactive Mode
If launched without arguments, `spurx-ipcalc` interactively prompts for input:
```bash
$ spurx-ipcalc
Enter IP / CIDR network (e.g., 192.168.1.0/24 or 2001:db8::/64): 10.0.0.0/8
```

---

## Running Tests

Run the test suite:
```bash
cargo test
```

Run linter:
```bash
cargo clippy -- -D warnings
```

---

## License

Dual-licensed under either of:
- Apache License, Version 2.0
- MIT License
