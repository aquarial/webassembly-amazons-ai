SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: build
build:
	nix-shell --command "wasm-pack build rust"

.PHONY: run-local
run-local:
	nix-shell --command "           \
	    wasm-pack build rust/    && \
	    cd website               && \
	    npm run start"

.PHONY: deploy
deploy:
	nix-shell --command "           \
	    wasm-pack build rust/    && \
	    cd website               && \
	    npx webpack"
	echo "Check ./wasm-app/dist/"


.PHONY: clean
clean:
	rm -rf rust/pkg
	rm -rf website/dist
