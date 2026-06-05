# cloud-netconfig

[![CI](https://github.com/hypersdk/cloud-netconfig/actions/workflows/ci.yml/badge.svg)](https://github.com/hypersdk/cloud-netconfig/actions/workflows/ci.yml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Release](https://img.shields.io/github/v/release/hypersdk/cloud-netconfig)](https://github.com/hypersdk/cloud-netconfig/releases)

<p align="center">
  <a href="https://zyvor.dev/demo?utm_source=github&utm_medium=cloud-netconfig"><img src="https://img.shields.io/badge/Demo-F97316?style=flat-square" alt="Demo"/></a>
  <a href="https://zyvor.dev/contact?utm_source=github&utm_medium=cloud-netconfig"><img src="https://img.shields.io/badge/Contact_sales-22C55E?style=flat-square" alt="Contact"/></a>
  <a href="https://github.com/sponsors/ssahani?utm_source=github&utm_medium=cloud-netconfig"><img src="https://img.shields.io/badge/Sponsor-ssahani-ea4aaa?style=flat-square&logo=github-sponsors&logoColor=white" alt="Sponsor"/></a>
</p>

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

## Support the project

cloud-netconfig Community Edition is free and open source. If it saves you time, consider sponsoring ongoing development:

[![Sponsor on GitHub](https://img.shields.io/badge/Sponsor-ssahani-ea4aaa?logo=github-sponsors&logoColor=white)](https://github.com/sponsors/ssahani?utm_source=github&utm_medium=cloud-netconfig)

Maintained by **Susant Sahani** · [Zyvor AI Labs](https://zyvor.dev?utm_source=github&utm_medium=cloud-netconfig)

- **Sponsor:** [github.com/sponsors/ssahani](https://github.com/sponsors/ssahani?utm_source=github&utm_medium=cloud-netconfig)
- **Enterprise / production:** [zyvor.dev/contact](https://zyvor.dev/contact?utm_source=github&utm_medium=cloud-netconfig) · [sales@zyvor.dev](mailto:sales@zyvor.dev)
- **Community help:** [GitHub Issues](https://github.com/hypersdk/cloud-netconfig/issues)

## License

cloud-netconfig is licensed under the Apache License, Version 2.0.

Copyright © 2026 Zyvor AI Labs Private Limited.

This repository contains only the cloud-netconfig Community Edition source code.

Other Zyvor products, platforms, services, and commercial offerings are separate works and may be governed by different licenses and terms.

Enterprise: [sales@zyvor.dev](mailto:sales@zyvor.dev) · General: [info@zyvor.dev](mailto:info@zyvor.dev). Security: [SECURITY.md](SECURITY.md).

Related: [netevd](https://github.com/hypersdk/netevd) · [netctl](https://github.com/hypersdk/netctl) · [hypersdk](https://github.com/hypersdk/hypersdk)
