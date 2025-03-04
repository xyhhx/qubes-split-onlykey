# Qubes Split Onlykey

[![Hippocratic License HL3-FULL](https://img.shields.io/static/v1?label=Hippocratic%20License&message=HL3-FULL&labelColor=5e2751&color=bc8c3d)](https://firstdonoharm.dev/version/3/0/full.html)

> [!Note]
> Still in early development

> [!Warning]
> Even `main` might be cooked

---

### Design goals

- Provision separate sockets and configurations for each client domain
- Isolate sockets and configurations using systemd sandboxing

---

#### Acknowledgements

Design inspiration mostly coming from:

- https://piware.de/post/2019-10-15-cockpit-systemd-activation-cubed/
- https://github.com/cockpit-project/cockpit/blob/main/src/tls/README.md
- https://gist.github.com/bcduggan/bb60d79d2d1a2c2045d3a5dd4d35ca4d
