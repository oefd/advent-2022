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

.PHONY: day03
day03: build
	@echo -e "\033[1;36m== day03 ==\033[0m\033[0;34m"
	@./target/release/day03 < day03/test_input | sed 's/^/  /'

.PHONY: day04
day04: build
	@echo -e "\033[1;36m== day04 ==\033[0m\033[0;34m"
	@./target/release/day04 < day04/test_input | sed 's/^/  /'

.PHONY: day05
day05: build
	@echo -e "\033[1;36m== day05 ==\033[0m\033[0;34m"
	@./target/release/day05 < day05/test_input | sed 's/^/  /'

.PHONY: day06
day06: build
	@echo -e "\033[1;36m== day06 ==\033[0m\033[0;34m"
	@./target/release/day06 < day06/test_input | sed 's/^/  /'

.PHONY: day07
day07: build
	@echo -e "\033[1;36m== day07 ==\033[0m\033[0;34m"
	@./target/release/day07 < day07/test_input | sed 's/^/  /'

.PHONY: day08
day08: build
	@echo -e "\033[1;36m== day08 ==\033[0m\033[0;34m"
	@./target/release/day08 < day08/test_input | sed 's/^/  /'

.PHONY: day09
day09: build
	@echo -e "\033[1;36m== day09 ==\033[0m\033[0;34m"
	@./target/release/day09 < day09/test_input | sed 's/^/  /'

.PHONY: day10
day10: build
	@echo -e "\033[1;36m== day10 ==\033[0m\033[0;34m"
	@./target/release/day10 < day10/test_input | sed 's/^/  /'

.PHONY: day11
day11: build
	@echo -e "\033[1;36m== day11 ==\033[0m\033[0;34m"
	@./target/release/day11 < day11/test_input | sed 's/^/  /'
