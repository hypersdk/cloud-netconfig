# cloud-netconfig

[![CI](https://github.com/hypersdk/cloud-netconfig/actions/workflows/ci.yml/badge.svg)](https://github.com/hypersdk/cloud-netconfig/actions/workflows/ci.yml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Release](https://img.shields.io/github/v/release/hypersdk/cloud-netconfig)](https://github.com/hypersdk/cloud-netconfig/releases)

Automatic network configuration for cloud instances using provider metadata (Azure, AWS, GCP, and others). Handles secondary IPs, routing tables, and policy-based routing on multi-interface VMs.

## Features

- Multi-cloud metadata clients (Azure, AWS EC2, GCP, and more)
- Event-driven reconfiguration via netlink
- Policy-based routing for multi-homed hosts
- Local HTTP API for instance metadata
- Runs unprivileged with `CAP_NET_ADMIN`

## Installation

```bash
git clone https://github.com/hypersdk/cloud-netconfig.git
cd cloud-netconfig
make build
sudo make install
sudo useradd -M -s /usr/bin/nologin cloud-network 2>/dev/null || true
sudo systemctl enable --now cloud-netconfigd
```

## Configuration

Default path: `/etc/cloud-network/cloud-network.yaml`

```yaml
logging:
  level: info
  format: text

server:
  listen:
    address: 127.0.0.1
    port: 5209

metadata:
  refresh_interval: 300s
  request_timeout: 10s

network:
  interfaces:
    enabled:
      - eth1
      - eth2
  routing:
    table_base: 9999
    policy_routing: true
```

Annotated reference: [distribution/etc/cloud-network/config.yaml](distribution/etc/cloud-network/config.yaml).

## CLI

```bash
cnctl status system
cnctl show interfaces
```

## Development

```bash
cargo build --release
cargo test
```

## Troubleshooting

```bash
sudo journalctl -u cloud-netconfigd -f
cnctl status system
```

Enable debug logging in the config file (`logging.level: debug`) when diagnosing metadata or routing issues.

## Enterprise

| | |
|---|---|
| **Demo** | [zyvor.dev/demo](https://zyvor.dev/demo?utm_source=github&utm_medium=cloud-netconfig) |
| **ROI** | [zyvor.dev/roi](https://zyvor.dev/roi?utm_source=github&utm_medium=cloud-netconfig) |
| **Pricing** | [zyvor.dev/pricing](https://zyvor.dev/pricing?utm_source=github&utm_medium=cloud-netconfig) |
| **Contact** | [zyvor.dev/contact](https://zyvor.dev/contact?utm_source=github&utm_medium=cloud-netconfig) · [sales@zyvor.dev](mailto:sales@zyvor.dev) |

Community Edition is the open-source daemon. Supported multi-cloud rollouts, SLAs, and HyperSDK migration integration → contact Zyvor (not GitHub Issues). Details: [docs/enterprise.md](docs/enterprise.md).

## License

cloud-netconfig Community Edition is licensed under the Apache License 2.0.
cloud-netconfig Enterprise Edition includes additional proprietary features and is licensed
separately under a commercial license from Zyvor AI Labs Private Limited.

Enterprise: [sales@zyvor.dev](mailto:sales@zyvor.dev) · General: [info@zyvor.dev](mailto:info@zyvor.dev). Security: [SECURITY.md](SECURITY.md).

## Support

[github.com/hypersdk/cloud-netconfig/issues](https://github.com/hypersdk/cloud-netconfig/issues)

Related: [netevd](https://github.com/hypersdk/netevd) · [netctl](https://github.com/hypersdk/netctl) · [hypersdk](https://github.com/hypersdk/hypersdk)
