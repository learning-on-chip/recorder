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
	@find $(target)/build -name lib*.$(library_suffix)* -exec cp {} $(root)/lib/ \;
	@echo 'Well done! Now set your environment variables:'
	@echo $(color)'export BULLET_ROOT="$(root)"'
	@echo 'export PATH="$$BULLET_ROOT/bin:$$PATH"'
	@echo 'export $(library_path)="$$BULLET_ROOT/lib:$$$(library_path)"'

$(binary): $(sources)
	@cargo build --release

.PHONY: build install
