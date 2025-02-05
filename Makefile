OUTDIR           ?= $(CURDIR)/_out
DOCS_DIR         ?= /usr/share/doc
POSTINSTALL_DIR  ?= /etc/qubes/post-install.d
QREXEC_DIR       ?= /etc/qubes-rpc
QUBES_BIN_DIR    ?= /usr/lib/qubes
RPC_POLICIES_DIR ?= /etc/qubes/policy.d
UDEV_RULES_DIR   ?= /usr/lib/udev/rules.d

.PHONY: install-vm
install-vm:
	install -d $(DESTDIR)$(QREXEC_DIR)
	install -d $(DESTDIR)$(QUBES_BIN_DIR)/split-onlykey
	install -d $(DESTDIR)$(UDEV_RULES_DIR)
	install -d $(DESTDIR)$(DOCS_DIR)/split-onlykey
	install -d $(DESTDIR)$(POSTINSTALL_DIR)
	install -m 0755 \
		src/vm/onlykey.SshAgent \
		$(DESTDIR)$(QREXEC_DIR)
	install -m 0644 \
		src/vm/49-onlykey.rules \
		$(DESTDIR)$(UDEV_RULES_DIR)
	install -m 0644 \
		README.md \
		$(DESTDIR)$(DOCS_DIR)/split-onlykey
	# install -m 0755 \
	# 	post-install.d/*.sh \
	# 	$(DESTDIR)$(POSTINSTALL_DIR) 
	install -m 0755 \
		src/vm/ok-proxy-ssh-agent \
		$(DESTDIR)$(QUBES_BIN_DIR)/

.PHONY: install-dom0
install-dom0:
	install -D -m 0664 \
		src/dom0/49-onlykey.policy \
		$(DESTDIR)$(RPC_POLICIES_DIR)/49-onlykey.policy

