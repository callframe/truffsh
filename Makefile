## Makefile configure
.DEFAULT_GOAL := all
Q_FLAG := @

## Paths
WORKING_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
SOURCE_DIR := $(WORKING_DIR)/src

## Options
Q ?= $(Q_FLAG)

## Toolchains
ECHO := echo
RM := rm
CMAKE := cmake

define notice
$(Q_FLAG)$(ECHO) " $1 " $(notdir $(2))
endef

## Dependencies
MIMALLOC_DIR := $(WORKING_DIR)/mimalloc
MIMALLOC_INCLUDE := $(MIMALLOC_DIR)/include
MIMALLOC_BUILD_DIR := $(MIMALLOC_DIR)/build
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
RM_FLAGS := -rf
CMAKE_FLAGS := -G"Unix Makefiles"

## Rules
.PHONY: all
all: 

$(MIMALLOC_BUILD_DIR):
	$(call notice,CMAKE,$@)
	$(Q)CC=$(CC) $(CMAKE) -S $(MIMALLOC_DIR) -B $@ $(MIMALLOC_FLAGS)

$(MIMALLOC_OBJECT): | $(MIMALLOC_BUILD_DIR)
	$(call notice,MAKE,$@)
	$(Q)$(MAKE) -C $(MIMALLOC_BUILD_DIR)

.PHONY: clean-mimalloc
clean-mimalloc:
	$(call notice,RM,$(MIMALLOC_BUILD_DIR))
	$(Q)$(RM) $(RM_FLAGS) $(MIMALLOC_BUILD_DIR)

.PHONY: clean
clean: clean-mimalloc
