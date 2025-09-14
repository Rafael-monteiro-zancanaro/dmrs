COMPILER=cargo
.PHONY = test build check

test:
	$(COMPILER) test

build:
	$(COMPILER) build

check:
	$(COMPILER) check