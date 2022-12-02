all: build

.PHONY: build
build:
	cargo build --release --no-default-features

.PHONY: day01
day01: build
	@echo -e "\033[1;36m== day01 ==\033[0m\033[0;34m"
	@./target/release/day01 < day01/test_input | sed 's/^/  /'

.PHONY: day02
day02: build
	@echo -e "\033[1;36m== day02 ==\033[0m\033[0;34m"
	@./target/release/day02 < day02/test_input | sed 's/^/  /'
	@echo -e "\033[1;36m== day02-traits ==\033[0m\033[0;34m"
	@./target/release/day02-traits < day02/test_input | sed 's/^/  /'
