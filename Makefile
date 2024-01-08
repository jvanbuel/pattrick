.PHONY: build linux-arm linux-x86 macos-m1 release

NAME = "pattrick"

build:
	cross build --release --target x86_64-apple-darwin
	cp target/x86_64-apple-darwin/release/$(NAME) $(NAME)

linux-arm:
	cross build --release --target aarch64-unknown-linux-gnu

linux-x86:
	cross build --release --target x86_64-unknown-linux-gnu

macos-m1:
	cross build --release --target aarch64-apple-darwin

release: build linux-arm linux-x86 macos-m1
	tar -zcvf target/$(NAME)-aarch64-unknown-linux-gnu.tar.gz -C target/aarch64-unknown-linux-gnu/release $(NAME)
	tar -zcvf target/$(NAME)-x86_64-unknown-linux-gnu.tar.gz -C target/x86_64-unknown-linux-gnu/release $(NAME)
	tar -zcvf target/$(NAME)-aarch64-apple-darwin.tar.gz -C target/aarch64-apple-darwin/release $(NAME)
	tar -zcvf target/$(NAME)-x86_64-apple-darwin.tar.gz -C target/x86_64-apple-darwin/release $(NAME)

smart-release:
	cargo smar-release pattrick --update-crates-index --execute
