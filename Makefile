VERSION := 0.1.0-beta.0

tag:
	git tag -m "v${VERSION}" v${VERSION}
	git push --tags

workflow:
	cargo build --release
	make tag
	cargo publish
