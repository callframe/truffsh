## Sources
NEOSH_DIR := $(MODULES_DIR)/neosh
NEOSH_SOURCE := $(NEOSH_DIR)/main.rs
NEOSH_OUTPUT := $(BUILD_DIR)/neosh
NEOSH_DEPEND := $(NEOSH_OUTPUT).d
NEOSH_FLAGS := \
	--edition=2024 \
	--emit=link,dep-info=$(NEOSH_DEPEND) \
	-C link-arg=-lc \
	-C link-arg=$(MIMALLOC_OBJECT) \
	-C panic=abort \
	-C lto=thin

$(NEOSH_OUTPUT): $(NEOSH_SOURCE) | $(MIMALLOC_OBJECT)
	$(call notice,RUSTC,$@)
	$(Q)$(RUSTC) $< -o $@ $(NEOSH_FLAGS)

.PHONY: clean-neosh
clean-neosh:
	$(call notice,CLEAN,$(NEOSH_OUTPUT))
	$(Q)$(RM) $(RM_FLAGS) $(NEOSH_OUTPUT) $(NEOSH_DEPEND)

-include $(NEOSH_DEPEND)
