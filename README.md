# Qubes Split Onlykey

a simple split ssh implementation for your onlykeys

---

[![Hippocratic License HL3-FULL](https://img.shields.io/static/v1?label=Hippocratic%20License&message=HL3-FULL&labelColor=5e2751&color=bc8c3d)](https://firstdonoharm.dev/version/3/0/full.html)

### setting up

#### create the onlykey agent template

- create a template for your onlykey agent. i like to call mine `provides-onlykey` and this repo assumes it's based on fedora 40 minimal
- copy `./src/server/install.sh` into it, and run that. this will install all the required dependencies
- copy `./src/server/qubes.SshAgent` into `/etc/qubes-rpc/qubes.SshAgent`

#### create a sys-onlykey appvm

- simply create an appvm based on the template you just created

#### configure your client vms' templates

- copy `./src/client/ssh-socat` into `/usr/lib/ssh-socat`
- give it a mode of `755`

### usage

- start your usb qube as usual, as well as your onlykey-agent appvm, and your client/dev appvm
- pass it to your usb qube (with the qubes usb proxy)
- in your client/dev appvm, run `. <(ssh-socat $identity $onlykey_vm)`, where `$identity` is the ssh identity you set up with your only-agent already, and `$onlykey_vm` is the name of your onlykey's appvm
  - alternatively, it will pick up the `$SSH_IDENTITY` and `$SSH_VAULT_VM` environment variables, which works really nicely in conjunction with [direnv](https://direnv.net)
