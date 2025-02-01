BUILD_DIR := _out
BUILD_TARBALL := $(BUILD_DIR)/sys-onlykey.tar.xz

.DEFAULT_GOAL: build 

build: $(BUILD_TARBALL)

$(BUILD_TARBALL):
	mkdir -p $(BUILD_DIR)/includes
	cp -r ./src/{dom0,client,server}/* _out/includes/
	cp -r ./srv/salt/* _out
	tar cjvf $(BUILD_TARBALL) _out/includes _out/*.sls _out/*.top

.PHONY: clean
clean:
	rm -rf $(BUILD_DIR)
