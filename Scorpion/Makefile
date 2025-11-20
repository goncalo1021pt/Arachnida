all: build

build:
	cargo build --release

run:
	cargo run --release

clean:

fclean: clean
	cargo clean
	rm -rf target/

re: fclean all

.PHONY: all build run clean fclean re