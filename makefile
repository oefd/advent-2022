all: day01

.PHONY: day01
day01:
	@echo -e "\033[1;36m== day01 ==\033[0m"
	cargo build --release --no-default-features
	@echo -e "\033[0;34m"
	@./target/release/day01 < day01/test_input | sed 's/^/  /'
	@echo -e "\033[0m"
