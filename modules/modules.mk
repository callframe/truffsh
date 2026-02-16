## Sources
NEOFFI_DIR := $(MODULES_DIR)/neo-ffi
NEOFFI_SOURCE := $(NEOFFI_DIR)/lib.rs
NEOFFI_OUTPUT := $(BUILD_DIR)/libneo-ffi.rlib
NEOFFI_DEPEND := $(NEOFFI_OUTPUT).d
NEOFFI_FLAGS := $(RUST_FLAGS) \
	--emit=link,dep-info=$(NEOFFI_DEPEND) \
	--crate-type=rlib

$(NEOFFI_OUTPUT): $(NEOFFI_SOURCE)
	$(call notice,RUSTC,$@)
	$(Q)$(RUSTC) $(NEOFFI_FLAGS) $< -o $@

NEOSH_DIR := $(MODULES_DIR)/neosh
NEOSH_SOURCE := $(NEOSH_DIR)/main.rs
NEOSH_OUTPUT := $(BUILD_DIR)/neosh
NEOSH_DEPEND := $(NEOSH_OUTPUT).d
NEOSH_FLAGS := $(RUST_FLAGS) \
	--emit=link,dep-info=$(NEOSH_DEPEND) \
	--crate-type=bin \
	-C link-arg=$(MIMALLOC_OBJECT) \
	-C link-arg=-lc \
	-C link-arg=-lgcc \
	--extern neo_ffi=$(NEOFFI_OUTPUT)

$(NEOSH_OUTPUT): $(NEOSH_SOURCE) $(NEOFFI_OUTPUT) | $(MIMALLOC_OBJECT)
	$(call notice,RUSTC,$@)
	$(Q)$(RUSTC) $(NEOSH_FLAGS) $< -o $@


.PHONY: clean-neoffi
clean-neoffi:
	$(call notice,CLEAN,$(NEOFFI_OUTPUT))
	$(Q)$(RM) $(RM_FLAGS) $(NEOFFI_OUTPUT)

.PHONY: clean-neosh
clean-neosh:
	$(call notice,CLEAN,$(NEOSH_OUTPUT))
	$(Q)$(RM) $(RM_FLAGS) $(NEOSH_OUTPUT) $(NEOSH_DEPEND)

.PHONY: clean-modules
clean-modules: clean-neosh clean-neoffi

-include $(NEOSH_DEPEND)
-include $(NEOFFI_DEPEND)
