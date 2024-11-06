#!/bin/sh
set -xeuo pipefail

export https_proxy="127.0.0.1:8082"
export PIP_PROXY="${https_proxy}"

dnf install -y python3-pip \
	python3-devel \
    python3-tkinter \
    libusb1-devel \
    libudev-devel \
    gcc \
    redhat-rpm-config \
    qubes-usb-proxy

pip3 install onlykey onlykey-agent

cp -r /usr/local/{bin,lib,lib64} /usr/local.orig

curl -sL https://raw.githubusercontent.com/trustcrypto/trustcrypto.github.io/pages/49-onlykey.rules > /etc/udev/rules.d/49-onlykey.rules

udevadm control --reload-rules
udevadm trigger
