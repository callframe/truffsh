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

ifeq ($(WITH_TC), $(CLANG_NAME))
	CC := clang
else ifeq ($(WITH_TC), $(GCC_NAME))
	CC := gcc
endif

define notice
$(Q_FLAG)$(ECHO) " $1 " $(notdir $(2))
endef

## Flags
CC_FLAGS := \
	-std=c11 \
	-Wall -Wextra -Werror \
	-MMD -MP \
	-I$(WORKING_DIR) -I$(SOURCE_DIR)
LD_FLAGS :=
RM_FLAGS := -rf
SED_FLAGS := -e

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
NEOSH_DEPENDS := $(NEOSH_SOURCES:.c=.d)

## Rules
.PHONY: all
all: $(BIN_NEOSH)

$(CONFIG_H): $(CONFIG_H_IN)
	$(call notice,SED,$@)
	$(Q)$(SED) $(SED_FLAGS) $< > $@

$(BIN_NEOSH): $(NEOSH_OBJECTS)
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


.PHONY: clean
clean: clean-objects clean-binaries

-include $(NEOSH_DEPENDS)