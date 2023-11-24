.PHONY: docs docs-open

docs:
	RUSTDOCFLAGS="--html-in-header katex-header.html --cfg docsrs" cargo +nightly doc --no-deps --all-features

docs-open:
	RUSTDOCFLAGS="--html-in-header katex-header.html --cfg docsrs" cargo +nightly doc --no-deps --all-features --open

docs-private:
	RUSTDOCFLAGS="--html-in-header katex-header.html --cfg docsrs" cargo +nightly doc --no-deps --all-features --document-private-items

readme:
	cargo rdme -w generic-ec -r README.md
	cargo rdme -w generic-ec-core -r generic-ec-core/README.md
	cargo rdme -w generic-ec-curves -r generic-ec-curves/README.md
	cargo rdme -w generic-ec-zkp -r generic-ec-zkp/README.md
