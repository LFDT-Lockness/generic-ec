.PHONY: docs docs-open

docs:
	RUSTDOCFLAGS="--html-in-header katex-header.html --cfg docsrs" cargo +nightly doc --no-deps --all-features

docs-open:
	RUSTDOCFLAGS="--html-in-header katex-header.html --cfg docsrs" cargo +nightly doc --no-deps --all-features --open

docs-private:
	RUSTDOCFLAGS="--html-in-header katex-header.html --cfg docsrs" cargo +nightly doc --no-deps --all-features --document-private-items

readme:
	(cd generic-ec; cargo rdme -r ../README.md)
	(cd generic-ec-core; cargo rdme -r README.md)
	(cd generic-ec-curves; cargo rdme -r README.md)
	(cd generic-ec-zkp; cargo rdme -r README.md)
