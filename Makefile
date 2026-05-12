.DEFAULT_GOAL := all
SHELL := /bin/bash

# ── helpers ────────────────────────────────────────────────────────────────────

BOLD  := $(shell tput bold 2>/dev/null)
RESET := $(shell tput sgr0 2>/dev/null)
GREEN := $(shell tput setaf 2 2>/dev/null)
RED   := $(shell tput setaf 1 2>/dev/null)
CYAN  := $(shell tput setaf 6 2>/dev/null)

ok  = echo "$(GREEN)✓ $1$(RESET)"
err = { echo "$(RED)✗ $1$(RESET)"; exit 1; }

# ── all ────────────────────────────────────────────────────────────────────────

.PHONY: all
all: setup lint test ## Setup complet + qualité + tests (défaut)

# ── setup ──────────────────────────────────────────────────────────────────────

.PHONY: setup
setup: _check-rustup ## Installe les composants et les hooks git
	@rustup component add clippy rustfmt
	@$(call ok,composants Rust installés)
	@if command -v pre-commit &>/dev/null; then \
		pre-commit install; \
		$(call ok,hooks pre-commit installés); \
	else \
		echo "$(RED)pre-commit introuvable$(RESET) — installez-le avec : pip install pre-commit"; \
	fi

.PHONY: _check-rustup
_check-rustup:
	@command -v rustup &>/dev/null || $(call err,rustup est absent — installez-le depuis https://rustup.rs)
	@$(call ok,rustup $(shell rustup --version 2>/dev/null | head -1))

# ── build / check ──────────────────────────────────────────────────────────────

.PHONY: build
build: ## Compile tous les exercices
	cargo build --workspace

.PHONY: check
check: ## Vérifie tous les exercices (sans compiler les binaires)
	cargo check --workspace

.PHONY: test
test: ## Lance tous les tests
	cargo test --workspace

# ── qualité ────────────────────────────────────────────────────────────────────

.PHONY: fmt
fmt: ## Formate le code
	cargo fmt --all

.PHONY: fmt-check
fmt-check: ## Vérifie le formatage sans modifier les fichiers
	cargo fmt --all -- --check

.PHONY: clippy
clippy: ## Analyse statique (clippy)
	cargo clippy --workspace -- -D warnings

.PHONY: lint
lint: fmt clippy ## fmt + clippy

# ── exercices ──────────────────────────────────────────────────────────────────

.PHONY: new
new: ## Crée un exercice  —  make new name=ex12-foo phase=phase2
	@test -n "$(name)"  || $(call err,précise un nom : make new name=ex12-foo phase=phase2)
	@test -n "$(phase)" || $(call err,précise une phase : make new name=ex12-foo phase=phase2)
	@mkdir -p exercises/$(phase)
	@cargo new --vcs none exercises/$(phase)/$(name)
	@printf '\n[dev-dependencies]\nassert_cmd = { workspace = true }\n' \
		>> exercises/$(phase)/$(name)/Cargo.toml
	@mkdir -p exercises/$(phase)/$(name)/tests
	@printf 'use assert_cmd::Command;\n\n#[test]\nfn test_output() {\n    Command::cargo_bin("$(name)")\n        .unwrap()\n        .assert()\n        .success()\n        .stdout("");\n}\n' \
		> exercises/$(phase)/$(name)/tests/output.rs
	@$(call ok,exercice exercises/$(phase)/$(name) créé)

# ── utilitaires ────────────────────────────────────────────────────────────────

.PHONY: clean
clean: ## Supprime les artefacts de compilation
	cargo clean

.PHONY: update
update: ## Met à jour les dépendances et la toolchain
	rustup update
	cargo update

.PHONY: list
list: ## Liste les exercices du workspace
	@cargo metadata --no-deps --format-version 1 \
		| python3 -c "import sys,json; [print(' ', m['name']) for m in json.load(sys.stdin)['packages']]" \
		2>/dev/null \
		|| grep -oP '(?<=members = \[)[^\]]+' Cargo.toml | tr ',' '\n' | tr -d ' "'

# ── aide ───────────────────────────────────────────────────────────────────────

.PHONY: help
help: ## Affiche cette aide
	@echo ""
	@echo "$(BOLD)Commandes disponibles$(RESET)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*##' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*##"}; {printf "  $(CYAN)%-14s$(RESET) %s\n", $$1, $$2}'
	@echo ""
