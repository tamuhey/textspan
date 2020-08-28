.PHONY: build test
SO = textspan/textspan.cpython-38-darwin.so
${SO}: src/lib.rs
	poetry run maturin develop
test: ${SO}
	poetry run pytest tests
build: src/lib.rs textspan/*
	poetry run maturin build
