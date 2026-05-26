# cloud-netconfig

[![CI](https://github.com/hypersdk/cloud-netconfig/actions/workflows/ci.yml/badge.svg)](https://github.com/hypersdk/cloud-netconfig/actions/workflows/ci.yml)
[![License: LGPL-3.0-or-later](https://img.shields.io/badge/License-LGPL%203.0--or--later-blue.svg)](https://www.gnu.org/licenses/lgpl-3.0)
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

Community Edition is the open-source daemon. Supported multi-cloud rollouts, SLAs, and HyperSDK migration integration → contact Zyvor (not GitHub Issues).

## License

LGPL-3.0-or-later — see [LICENSE.txt](LICENSE.txt). Security: [SECURITY.md](SECURITY.md).

## Support

[github.com/hypersdk/cloud-netconfig/issues](https://github.com/hypersdk/cloud-netconfig/issues)

Related: [netevd](https://github.com/hypersdk/netevd) · [netctl](https://github.com/hypersdk/netctl) · [hypersdk](https://github.com/hypersdk/hypersdk)
