## Makefile configure
.DEFAULT_GOAL := all
MAKEFLAGS += --warn-undefined-variables
Q_FLAG := @

## Paths
WORKING_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
SOURCE_DIR := $(WORKING_DIR)/src

## Options
Q ?= $(Q_FLAG)
WITH_TC ?= clang
WITH_DEBUG ?= 0

## Constants
CLANG_NAME := clang
GCC_NAME := gcc

## Toolchains
ECHO := echo
RM := rm
SED := sed
CMAKE := cmake

ifeq ($(WITH_TC), $(CLANG_NAME))
	CC := clang
else ifeq ($(WITH_TC), $(GCC_NAME))
	CC := gcc
endif

define notice
$(Q_FLAG)$(ECHO) " $1 " $(notdir $(2))
endef

## Dependencies
MIMALLOC_DIR := $(WORKING_DIR)/mimalloc
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
CC_FLAGS := \
	-std=c11 \
	-Wall -Wextra -Werror \
	-MMD -MP \
	-I$(WORKING_DIR) -I$(SOURCE_DIR)
LD_FLAGS := 
RM_FLAGS := -rf
SED_FLAGS := -e
CMAKE_FLAGS := -G"Unix Makefiles"

## Configs
SED_FLAGS += "s/@NEOSH_DEBUG@/$(WITH_DEBUG)/g"

CONFIG_H_IN := $(WORKING_DIR)/config.h.in
CONFIG_H := $(WORKING_DIR)/config.h

NEOSH_DEBUG := "@NEOSH_DEBUG@"

## Targets
BIN_NEOSH := $(WORKING_DIR)/neosh
NEOSH_SOURCES := \
	$(SOURCE_DIR)/neosh.c \
	$(SOURCE_DIR)/vec.c

NEOSH_OBJECTS := $(NEOSH_SOURCES:.c=.o)
NEOSH_OBJECTS += $(MIMALLOC_OBJECT)
NEOSH_DEPENDS := $(NEOSH_SOURCES:.c=.d)

## Rules
.PHONY: all
all: $(BIN_NEOSH)

$(MIMALLOC_BUILD_DIR):
	$(call notice,CMAKE,$@)
	$(Q)CC=$(CC) $(CMAKE) -S $(MIMALLOC_DIR) -B $@ $(MIMALLOC_FLAGS)

$(MIMALLOC_OBJECT): | $(MIMALLOC_BUILD_DIR)
	$(call notice,MAKE,$@)
	$(Q)$(MAKE) -C $(MIMALLOC_BUILD_DIR)

$(CONFIG_H): $(CONFIG_H_IN)
	$(call notice,SED,$@)
	$(Q)$(SED) $(SED_FLAGS) $< > $@

$(BIN_NEOSH): $(NEOSH_OBJECTS) $(MIMALLOC_OBJECT)
	$(call notice,LD,$@)
	$(Q)$(CC) $(LD_FLAGS) -o $@ $^

$(NEOSH_OBJECTS): $(CONFIG_H)

%.o: %.c
	$(call notice,CC,$@)
	$(Q)$(CC) $(CC_FLAGS) -c -o $@ $<

.PHONY: clean-objects
clean-objects:
	$(call notice,RM,$(NEOSH_OBJECTS))
	$(Q)$(RM) $(RM_FLAGS) $(NEOSH_OBJECTS)
	$(call notice,RM,$(NEOSH_DEPENDS))
	$(Q)$(RM) $(RM_FLAGS) $(NEOSH_DEPENDS)
	$(call notice,RM,$(CONFIG_H))
	$(Q)$(RM) $(RM_FLAGS) $(CONFIG_H)

.PHONY: clean-binaries
clean-binaries:
	$(call notice,RM,$(BIN_NEOSH))
	$(Q)$(RM) $(RM_FLAGS) $(BIN_NEOSH)

.PHONY: clean-mimalloc
clean-mimalloc:
	$(call notice,RM,$(MIMALLOC_BUILD_DIR))
	$(Q)$(RM) $(RM_FLAGS) $(MIMALLOC_BUILD_DIR)

.PHONY: clean
clean: clean-objects clean-binaries clean-mimalloc

-include $(NEOSH_DEPENDS)