VERSION := 0.1.0-beta.4

tag:
	git tag -m "v${VERSION}" v${VERSION}
	git push --tags

workflow:
	cargo build --release
	cargo publish
	make tag
