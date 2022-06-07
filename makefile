SUBDIRS := scripts

all: build

build:
	

test: build
	$(MAKE) -C $(SUBDIRS) all
	cargo test --release

.PHONY: clean

clean: 
	cargo clean