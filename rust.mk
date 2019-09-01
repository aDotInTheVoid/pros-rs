RUST_ARCH=armv7r-none-eabi
# Theirs no debug mode flag so I'm stuck with this
RUST_MODE=debug
RUST_MODE_FLAG=
RUST_PKG_NAME=pros_rs

RUST_LIB=target/$(RUST_ARCH)/$(RUST_MODE)/lib$(RUST_PKG_NAME).a

.PHONY: $(RUST_LIB)
$(RUST_LIB): 
	- cargo build --target $(RUST_ARCH) $(RUST_MODE_FLAG)
	- $(OBJCOPY) -R .ARM.attributes $(RUST_LIB) $(RUST_LIB)

LIBRARIES+=$(RUST_LIB)