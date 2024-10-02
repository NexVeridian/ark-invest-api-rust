precommit:
	rustup update
	cargo update
	cargo check
	cargo fmt
	cargo t
	cargo clippy --fix --allow-dirty

check:
	rustup update
	cargo update
	nix flake update
	nix flake check
	cargo clippy --fix --allow-dirty
