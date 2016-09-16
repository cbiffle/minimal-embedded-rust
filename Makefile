target = thumbv7em-none-eabi
mode ?= debug
binary = build/$(target)/$(mode)/elf
linker_script = layout.ld
rustlib = target/$(target)/$(mode)/libemb1.a

.PHONY: all clean cargo

all: $(binary)

clean:
	@cargo clean
	@rm -f $(binary)

$(binary): cargo $(rustlib) $(linker_script)
	@mkdir -p build/$(target)/$(mode)
	@arm-none-eabi-gcc -Wl,--gc-sections -nostdlib -nodefaultlibs \
	  -o $(binary) \
	  $(rustlib) -T$(linker_script) 

cargo:
ifeq ($(mode),release)
	@cargo build --release
else
	@cargo build
endif
