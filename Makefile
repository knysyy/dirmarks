# Option
#============================================
INSTALL_PATH := $(HOME)/.cargo
F_PATH := $(HOME)/.zsh/completion
#============================================

# Task
#============================================
fmt:
	cargo +nightly fmt

install:
	cargo install --force --root ${INSTALL_PATH} --path .

completion:
	RUST_LOG=info cargo run completion zsh > ${F_PATH}/_dirmarks

debug:
    RUST_LOG=debug cargo ${ARG}
#============================================
