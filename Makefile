BINARY   = md2optex
EXAMPLE  ?= examples/ukazka.md
BUILDDIR  = build

.PHONY: all build release install uninstall test check fmt lint clean tex pdf preview

all: build

build:
	cargo build

release:
	cargo build --release

install:
	cargo install --path .

uninstall:
	cargo uninstall $(BINARY)

test:
	cargo test

check: fmt lint test

fmt:
	cargo fmt --check

lint:
	cargo clippy -- -D warnings

# Generate TeX from the example Markdown file into build/
tex:
	RUSTFLAGS="-A warnings" cargo build --quiet
	mkdir -p $(BUILDDIR)
	./target/debug/$(BINARY) $(EXAMPLE) -o $(BUILDDIR)/ukazka.tex
	@echo "Output: $(BUILDDIR)/ukazka.tex"

# Generate TeX and compile to PDF with OpTeX (silent; log in build/ukazka.log)
pdf: tex
	cd $(BUILDDIR) && optex -interaction=batchmode ukazka.tex >ukazka.stdout 2>&1 \
		|| { echo "OpTeX failed — see $(BUILDDIR)/ukazka.log:"; \
		     grep "^!" $(BUILDDIR)/ukazka.log || cat $(BUILDDIR)/ukazka.log; \
		     exit 1; }
	@echo "Output: $(BUILDDIR)/ukazka.pdf"

# Open PDF in the default viewer
preview: pdf
	xdg-open $(BUILDDIR)/ukazka.pdf

clean:
	cargo clean
	rm -rf $(BUILDDIR)
