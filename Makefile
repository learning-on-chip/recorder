target = $(PWD)/target/release
binary = $(target)/bullet
sources := $(shell find src -name *.rs)

ifneq ($(DESTDIR),)
root = $(DESTDIR)
else
root = $(target)/root
endif

ifeq ($(shell uname),Darwin)
library_path = DYLD_LIBRARY_PATH
library_suffix = dylib
else
library_path = LD_LIBRARY_PATH
library_suffix = so
endif

color = $(shell tput setaf 2 || echo)

build: $(binary)

install: $(binary)
	@mkdir -p $(root)/bin
	@mkdir -p $(root)/lib
	@cp $^ $(root)/bin/bullet
	@find $(target)/build -name *.$(library_suffix) -exec cp {} $(root)/lib/ \;
	@echo 'Well done! Now set your environment variables:'
	@echo
	@echo $(color)'    export BULLET_ROOT="$(root)"'
	@echo $(color)'    export PATH="$$BULLET_ROOT/bin:$$PATH"'
	@echo $(color)'    export $(library_path)="$$BULLET_ROOT/lib:$$$(library_path)"'
	@echo

$(binary): $(sources)
	cargo build --release

.PHONY: build install
