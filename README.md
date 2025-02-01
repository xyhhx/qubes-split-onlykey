# Qubes Split Onlykey

a simple split ssh implementation for your onlykeys

[![Hippocratic License HL3-FULL](https://img.shields.io/static/v1?label=Hippocratic%20License&message=HL3-FULL&labelColor=5e2751&color=bc8c3d)](https://firstdonoharm.dev/version/3/0/full.html)

---

### installation

**in a disposable vm**:

- clone this repo into a dispvm
- run `make` in the root directory
- copy the built tarball into dom0:
  ```sh
  qvm-run -p "${dispvm}" 'cat /home/user/qubes-split-onlykey/_out/sys-onlykey.tar.xz' > /tmp/sys-onlykey.tar.xz
  ```

**in dom0**:

- extract the tarball:
  ```sh
  cd /tmp
  tar xjvf sys-onlykey.tar.xz
  ```
- copy the salt files into `/srv/salt`
  ```sh
  sudo cp -r /tmp/srv/salt/* /srv/salt/
  ```
- enable and run the salt states:
  ```sh
  sudo qubesctl top.enable sys-onlykey
  sudo qubesctl --targets=dom0,provides-onlykey,sys-onlykey --show-output state.hightstate
  ```
