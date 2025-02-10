# Qubes Onlykey Proxy

[![Hippocratic License HL3-FULL](https://img.shields.io/static/v1?label=Hippocratic%20License&message=HL3-FULL&labelColor=5e2751&color=bc8c3d)](https://firstdonoharm.dev/version/3/0/full.html)

---

## Installation

- Create the template

  - Create a template based on Fedora that has all the Onlykey tooling installed
    - You can use `pipx` to install `onlykey` and `onlykey-agent` from the template
  - Install `./src/vm/onlykey.SshAgent` to `/etc/qubes-rpc/onlykey.SshAgent`, and make it executable
  - Install `./systemd/onlykey-agent.socket` and `./systemd/onlykey-agent.service` to `/etc/systemd/user/`
  - Install the udev rules

- Create the sys VM

  - Create an AppVM based on that template
  - As a user, run `systemctl --user enable onlykey-agent.socket` and `systemctl --user start onlykey-agent.socket`
  - You will also need to enable the `onlykey-agent.socket` service from the Qube's settings

- Install the RPC policy in dom0
  - Install the policy in `./src/dom0/49-onlykey.policy` to `/etc/qubes/policy.d/49-onlykey.policy`

## Usage

- Plug in your Onlykey
- Enter your PIN
- Start `sys-onlykey`
- Pass your Onlykey from your USB qube to `sys-onlykey`
- In your development (client) qube, start the proxy:

  ```sh
  ok-ssh-proxy # QUBES_ONLYKEY_DOMAIN must be set
  ```

### Adding identities

In `sys-onlykey`, add identities like so:

```sh
onlykey-agent "${identity}" >> "${XDG_CONFIG_DIR}/onlykey/ssh-agent.conf"
```
