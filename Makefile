# Option
#============================================
INSTALL_PATH := $(HOME)/.cargo
#============================================

# Task
#============================================
fmt:
	cargo +nightly fmt

install:
	cargo install --force --root ${INSTALL_PATH} --path .
#============================================
