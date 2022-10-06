.PHONY: build

NAME = "pattrick"

build:
	cargo build --release && cp target/release/$(NAME) $(NAME)
