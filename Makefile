.PHONY: test
test: build
	./main

.PHONY: build
build: main
	rustc main.rs

