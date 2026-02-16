## Sources
NEOSH_DIR := $(MODULES_DIR)/neosh
NEOSH_SOURCE := $(NEOSH_DIR)/main.rs
NEOSH_OUTPUT := $(BUILD_DIR)/neosh
NEOSH_DEPEND := $(NEOSH_OUTPUT).d
NEOSH_FLAGS := \
	-C link-args=$(MIMALLOC_OBJECT) \
	--extern mimalloc=$(MIMALLOC_OBJECT) \
	--emit=dep-info=$(NEOSH_DEPEND)

$(NEOSH_OUTPUT): $(NEOSH_SOURCE) | $(MIMALLOC_OBJECT)
	$(call notice,RUSTC,$@)
	$(Q)$(RUSTC) $< -o $@ $(NEOSH_FLAGS)

-include $(NEOSH_DEPEND)