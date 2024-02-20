.PHONY: docs docs-open

docs:
	RUSTDOCFLAGS="--html-in-header katex-header.html --cfg docsrs" cargo +nightly doc --no-deps --all-features --workspace --exclude nostd-example

docs-open:
	RUSTDOCFLAGS="--html-in-header katex-header.html --cfg docsrs" cargo +nightly doc --no-deps --all-features --workspace --exclude nostd-example --open

docs-private:
	RUSTDOCFLAGS="--html-in-header katex-header.html --cfg docsrs" cargo +nightly doc --no-deps --all-features --workspace --exclude nostd-example --document-private-items

readme:
	(cd generic-ec; cargo rdme -r ../README.md)
	(cd generic-ec-core; cargo rdme -r README.md)
	(cd generic-ec-curves; cargo rdme -r README.md)
	(cd generic-ec-zkp; cargo rdme -r README.md)
