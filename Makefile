# Colors
BLUE := \033[1;34m
GREEN := \033[1;32m
RESET := \033[0m

.PHONY: all build build-docker clean test check lint fmt fix-fmt

all:
	@echo "$(GREEN)Running all checks and building...$(RESET)"
	$(MAKE) fmt check build
	@echo "$(GREEN)All steps completed successfully!$(RESET)"

build:
	@echo "$(BLUE)Building release binary...$(RESET)"
	cargo build --release

check: lint test

lint:
	@echo "$(BLUE)Running clippy...$(RESET)"
	cargo clippy --all-targets --all-features -- -D warnings

test:
	@echo "$(BLUE)Running tests...$(RESET)"
	cargo test --all-targets --all-features

fmt:
	@echo "$(BLUE)Checking formatting...$(RESET)"
	cargo fmt --all -- --check

fix-fmt:
	@echo "$(BLUE)Applying formatting...$(RESET)"
	cargo fmt --all

build-docker:
	@echo "$(BLUE)Building Docker image...$(RESET)"
	docker build -t rustcloak-operator .

clean:
	@echo "$(BLUE)Cleaning...$(RESET)"
	cargo clean
