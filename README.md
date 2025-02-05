# Qubes Onlykey Proxy

[![Hippocratic License HL3-FULL](https://img.shields.io/static/v1?label=Hippocratic%20License&message=HL3-FULL&labelColor=5e2751&color=bc8c3d)](https://firstdonoharm.dev/version/3/0/full.html)

---

## Installation

TODO

## Usage

- Start `sys-onlykey`
- Pass your Onlykey from your USB qube to `sys-onlykey`
- In your development (client) qube, start the proxy:
  ```sh
  ok-proxy-ssh "${ssh_identity}" # ssh_identity should match an ssh identity on your onlykey
  ```
