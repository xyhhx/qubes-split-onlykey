# -*- coding: utf-8 -*-
# vim: set syntax=yaml ts=2 sw=2 sts=2 et :

---

{% if grains['id'] == 'dom0' %}

'fedora-41-minimal':
  qvm.template_installed

'provides-onlykey':
  qvm.vm:
    - clone:
      - source: fedora-41-minimal
    - prefs:
      - label: gray

'sys-onlykey':
  qvm.vm:
    - present:
      - template: provides-onlykey
      - label: yellow
      - netvm: none
    - service:
      - enable:
        - updates-proxy-setup

/etc/qubes/policy.d/49-onlykey.policy:
  file.managed:
    - source: salt://includes/49-onlykey.policy

{% elif grains['id'] == 'provides-onlykey' %}

'provides-onlykey__pkg.installed':
  pkg.installed:
    - pkgs:
      - gcc 
      - libudev-devel 
      - libusb1-devel 
      - pipx
      - qubes-core-agent-networking
      - qubes-core-agent-passwordless-root
      - qubes-ctap
      - qubes-usb-proxy
    - skip_suggestions: true
    - install_recommends: false

/etc/qubes-rpc/onlykey.SshAgent:
  file.managed:
    - source: salt://includes/onlykey.SshAgent
    - user: user
    - group: user
    - mode: 0755

/etc/udev/rules.d/49-onlykey.rules:
  file.managed:
    - makedirs: true
    - source: salt://includes/49-onlykey.rules

'udevadm control --reload-rules':
  cmd.run

'udevadm trigger':
  cmd.run

{% elif grains['id'] == 'sys-onlykey' %}

'HTTPS_PROXY=127.0.0.1:8082 pipx install onlykey onlykey-agent':
  cmd.run:
    - runas: user 

/usr/local/bin/ssh-socat:
  file.managed:
    - source: salt://includes/ssh-socat
    - user: user
    - group: user
    - mode: 0755

{% endif %}
