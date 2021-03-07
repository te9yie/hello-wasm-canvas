.DEFAULT_GOAL := help

.PHONY: build
build: ## Build wasm.
	wasm-pack build --target web

.PHONY: serve
serve: ## Run http server.
	npx serve

.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
