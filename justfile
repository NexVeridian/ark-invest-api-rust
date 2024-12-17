precommit:
	cargo update
	cargo check
	cargo fmt
	cargo t
	cargo clippy --fix --allow-dirty

check:
	cargo update
	nix flake update
	nix flake check
	cargo clippy --fix --allow-dirty
