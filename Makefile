## Makefile configure
.DEFAULT_GOAL := all
MAKEFLAGS += --warn-undefined-variables
Q_FLAG := @

## Paths
WORKING_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))
CONFIG_DIR := $(WORKING_DIR)/config
SOURCE_DIR := $(WORKING_DIR)/src

## Options
Q ?= $(Q_FLAG)
USE_TC ?= clang

## Constants
CLANG_NAME := clang
GCC_NAME := gcc

## Toolchains
ECHO := echo
RM := rm

ifeq ($(USE_TC), $(CLANG_NAME))
	CC := clang
else ifeq ($(USE_TC), $(GCC_NAME))
	CC := gcc
endif

define notice
$(Q_FLAG)$(ECHO) " $1 " $(notdir $(2))
endef

## Flags
CC_FLAGS := -Wall -Wextra -Werror
LD_FLAGS :=
RM_FLAGS := -rf

## Targets
BIN_NEOSH := $(WORKING_DIR)/neosh
NEOSH_SOURCES := \
	$(SOURCE_DIR)/neosh.c
NEOSH_OBJECTS := $(NEOSH_SOURCES:.c=.o)

## Rules
.PHONY: all
all: $(BIN_NEOSH)

$(BIN_NEOSH): $(NEOSH_OBJECTS)
	$(call notice,LD,$@)
	$(Q)$(CC) $(LD_FLAGS) -o $@ $^

%.o: %.c
	$(call notice,CC,$@)
	$(Q)$(CC) $(CC_FLAGS) -c -o $@ $<

.PHONY: clean-objects
clean-objects:
	$(call notice,RM,$(NEOSH_OBJECTS))
	$(Q)$(RM) $(RM_FLAGS) $(NEOSH_OBJECTS)

.PHONY: clean-binaries
clean-binaries:
	$(call notice,RM,$(BIN_NEOSH))
	$(Q)$(RM) $(RM_FLAGS) $(BIN_NEOSH)

.PHONY: clean
clean: clean-objects clean-binaries
