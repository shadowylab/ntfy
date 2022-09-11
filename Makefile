# Use 'DEBUG=1' to build debug binary'.
ifdef DEBUG
  GENERAL_ARGS := 
else
  GENERAL_ARGS := --release
endif

# Use 'VERBOSE=1' to echo all commands, for example 'make help VERBOSE=1'.
ifdef VERBOSE
  Q :=
else
  Q := @
endif

all: build

help:
	$(Q)echo ""
	$(Q)echo "make build             - Build executable"
	$(Q)echo "make precommit         - Execute precommit steps"
	$(Q)echo "make loc               - Count lines of code in src folder"
	$(Q)echo ""

build:
	$(Q)cargo build $(GENERAL_ARGS)

precommit:
	$(Q)cargo fmt && cargo clippy

clean:
	$(Q)cargo clean

loc:
	$(Q)echo "--- Counting lines of .rs files in 'src' (LOC):" && find src/ -type f -name "*.rs" -exec cat {} \; | wc -l
