#!/bin/sh
set -xeuo pipefail

PIPX_GLOBAL_BIN_DIR=/usr/bin HTTP_PROXY=127.0.0.1:8082 \
		pipx install onlykey onlykey-agent
