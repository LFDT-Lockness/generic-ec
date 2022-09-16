.PHONY: docs docs-open

docs:
	RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps

docs-open:
	RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps --open

docs-private:
	RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps --document-private-items
