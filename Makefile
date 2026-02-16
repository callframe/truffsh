## Makefile configure
.DEFAULT_GOAL := all
Q_FLAG := @

## Paths
WORKING_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
MODULES_DIR := $(WORKING_DIR)/modules
BUILD_DIR := $(WORKING_DIR)/build

RUST_PROJECT_IN := $(WORKING_DIR)/rust-project.json.in
RUST_PROJECT_OUT := $(WORKING_DIR)/rust-project.json

## Options
Q ?= $(Q_FLAG)
TOOLCHAIN ?= x86_64-unknown-linux-gnu

## Toolchains
ECHO := echo
RM := rm
SED := sed
CMAKE := cmake
MKDIR := mkdir
RUSTC := rustc

define notice
$(Q_FLAG)$(ECHO) " $1 " $(notdir $(2))
endef

## Dependencies
MIMALLOC_DIR := $(WORKING_DIR)/mimalloc
MIMALLOC_INCLUDE := $(MIMALLOC_DIR)/include
MIMALLOC_BUILD_DIR := $(BUILD_DIR)/mimalloc
MIMALLOC_OBJECT := $(MIMALLOC_BUILD_DIR)/mimalloc.o
MIMALLOC_FLAGS := \
	-DMI_BUILD_OBJECT=ON \
	-DMI_BUILD_TESTS=OFF \
	-DMI_BUILD_STATIC=OFF \
	-DMI_BUILD_SHARED=OFF \
	-DMI_XMALLOC=ON \
	-DMI_OVERRIDE=ON

## Flags
LD_FLAGS := 
SED_FLAGS := -e
RM_FLAGS := -rf
CMAKE_FLAGS := -G"Unix Makefiles"
RUST_FLAGS := \
	--target=$(TOOLCHAIN) \
	--edition=2024 \
	-C panic=abort \
	-C lto=thin

RUST_SYSROOT_FLAGS := --print sysroot

## Includes
include $(MODULES_DIR)/modules.mk

## Rules
.PHONY: all
all: $(RUST_PROJECT_OUT) $(NEOSH_OUTPUT)

$(RUST_PROJECT_OUT): SYSROOT := $(shell $(RUSTC) $(RUST_SYSROOT_FLAGS))
$(RUST_PROJECT_OUT): $(RUST_PROJECT_IN)
	$(call notice,CONFIG,$@)
	$(Q)$(SED) $(SED_FLAGS) "s|@SYSROOT@|$(SYSROOT)|g" $< > $@

$(BUILD_DIR):
	$(call notice,MKDIR,$@)
	$(Q)$(MKDIR) $@

$(MIMALLOC_BUILD_DIR): | $(BUILD_DIR)
	$(call notice,CMAKE,$@)
	$(Q)CC=$(CC) $(CMAKE) -S $(MIMALLOC_DIR) -B $@ $(MIMALLOC_FLAGS)

$(MIMALLOC_OBJECT): | $(MIMALLOC_BUILD_DIR)
	$(call notice,MAKE,$@)
	$(Q)$(MAKE) -C $(MIMALLOC_BUILD_DIR)

.PHONY: clean-mimalloc
clean-mimalloc:
	$(call notice,RM,$(MIMALLOC_BUILD_DIR))
	$(Q)$(RM) $(RM_FLAGS) $(MIMALLOC_BUILD_DIR)

.PHONY: clean-rust-project
clean-rust-project:
	$(call notice,RM,$(RUST_PROJECT_OUT))
	$(Q)$(RM) $(RM_FLAGS) $(RUST_PROJECT_OUT)

.PHONY: clean
clean: clean-modules clean-mimalloc clean-rust-project
