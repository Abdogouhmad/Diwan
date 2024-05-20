# Cargo command
CARGO := cargo

# Cargo options
BUILD := build --release
CLIPPY := clippy
FMT := fmt
RUN := run --
CLEAN := clean

# Target executable
TARGET := target/release/diwan

# Default rule
all: build

# Build the project
build:
	@echo "Building the project..."
	@$(CARGO) $(BUILD)

# Execute the project
exe: build
	@echo "Executing the project..."
	@$(CARGO) $(RUN)

# Lint the project
linter:
	@echo "Running Clippy (linter)..."
	@$(CARGO) $(CLIPPY)

# Format the project
format:
	@echo "Formatting the code..."
	@$(CARGO) $(FMT)

# Install the CLI
install: build
	@echo "Installing the CLI..."
	@sudo cp $(TARGET) /usr/bin

# Uninstall the CLI
uninstall:
	@echo "Uninstalling the CLI..."
	@sudo rm /usr/bin/diwan

# Clean the project
clean:
	@echo "Cleaning the project..."
	@$(CARGO) $(CLEAN)

# Phony targets to prevent conflicts with files of the same name
.PHONY: all build exe linter format install uninstall clean
