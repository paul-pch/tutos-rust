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

# `#` reste un début de commentaire jusque dans un `define` — il faut l'échapper
# pour pouvoir écrire des attributs Rust (`#[test]`) dans les gabarits.
HASH := \#

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

.PHONY: bench
bench: ## Lance les benchmarks (exercices qui en ont)
	cargo bench --workspace

.PHONY: doc
doc: ## Génère et ouvre la doc des exercices
	cargo doc --workspace --no-deps --open

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

# Squelette d'exercice (phase 3 et suivantes) : une lib testable + un binaire mince.
#   src/lib.rs  → la logique, couverte par des tests unitaires
#   src/main.rs → parse l'entrée, appelle la lib, affiche
#   tests/api.rs    → tests d'intégration sur l'API publique de la lib
#   tests/output.rs → assertion sur le binaire (exacte, ou par prédicat si non déterministe)

exdir   = exercises/$(phase)/$(name)
libname = $(subst -,_,$(name))

define TPL_CARGO
[package]
name = "$(name)"
version = "0.1.0"
edition.workspace = true

[lib]
name = "$(libname)"
path = "src/lib.rs"

[dependencies]

[dev-dependencies]
assert_cmd = { workspace = true }
predicates = { workspace = true }
endef

define TPL_LIB
//! $(name) — la logique de l'exercice.
//!
//! Tout ce qui se teste vit ici ; `src/main.rs` ne fait qu'appeler et afficher.
//! La lib s'importe sous le nom `$(libname)` (les tirets deviennent des underscores).

$(HASH)[cfg(test)]
mod tests {
    // Les tests unitaires de la logique se placent ici.
}
endef

define TPL_MAIN
fn main() {
    println!("Hello, world!");
}
endef

define TPL_API
//! Tests d'intégration : ils ne voient que l'API publique de la lib,
//! exactement comme le ferait un utilisateur — `use $(libname)::...;`
endef

define TPL_OUTPUT
use assert_cmd::Command;

$(HASH)[test]
fn test_output() {
    Command::cargo_bin("$(name)")
        .unwrap()
        .assert()
        .success()
        .stdout("");
}
endef

define TPL_BENCH
use criterion::{Criterion, criterion_group, criterion_main};

fn benchmarks(c: &mut Criterion) {
    let _ = c;
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
endef

# Exportés dans l'environnement du shell : `$(file …)` serait évalué à l'expansion
# de la recette, donc avant que `mkdir` ait créé les répertoires.
export TPL_CARGO TPL_LIB TPL_MAIN TPL_API TPL_OUTPUT TPL_BENCH

.PHONY: new
new: ## Crée un exercice  —  make new name=ex17-foo phase=phase3 [bench=1]
	@test -n "$(name)"  || $(call err,précise un nom : make new name=ex17-foo phase=phase3)
	@test -n "$(phase)" || $(call err,précise une phase : make new name=ex17-foo phase=phase3)
	@test ! -e $(exdir)   || $(call err,$(exdir) existe déjà)
	@mkdir -p $(exdir)/src $(exdir)/tests
	@printf '%s\n' "$$TPL_CARGO"  > $(exdir)/Cargo.toml
	@printf '%s\n' "$$TPL_LIB"    > $(exdir)/src/lib.rs
	@printf '%s\n' "$$TPL_MAIN"   > $(exdir)/src/main.rs
	@printf '%s\n' "$$TPL_API"    > $(exdir)/tests/api.rs
	@printf '%s\n' "$$TPL_OUTPUT" > $(exdir)/tests/output.rs
	@if [ -n "$(bench)" ]; then \
		mkdir -p $(exdir)/benches; \
		printf '%s\n' "$$TPL_BENCH" > $(exdir)/benches/bench.rs; \
		printf 'criterion = { workspace = true }\n\n[[bench]]\nname = "bench"\nharness = false\n' >> $(exdir)/Cargo.toml; \
		$(call ok,benches/bench.rs ajouté); \
	fi
	@grep -q '"exercises/$(phase)/\*"' Cargo.toml \
		|| echo "$(RED)⚠ ajoute \"exercises/$(phase)/*\" aux members du Cargo.toml racine$(RESET)"
	@$(call ok,exercice $(exdir) créé)

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
