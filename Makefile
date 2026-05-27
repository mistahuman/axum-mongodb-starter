.PHONY: help install dev test build run clean mongo mongo-stop

APP_NAME        := axum-mongodb-starter
PORT            := 8000
MONGO_CONTAINER := mongo-dev
MONGO_PORT      := 27017

help: ## Show available commands
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-10s\033[0m %s\n", $$1, $$2}'

install: ## Fetch and compile dependencies
	cargo fetch

dev: ## Run development server
	cargo run

test: ## Run integration tests (requires running server)
	cargo test

build: ## Build Docker image
	docker build -t $(APP_NAME) .

run: ## Run app via Docker
	docker run --env-file .env -p $(PORT):$(PORT) $(APP_NAME)

mongo: ## Start a local MongoDB container
	docker run -d --name $(MONGO_CONTAINER) -p $(MONGO_PORT):27017 mongo:8 \
		|| docker start $(MONGO_CONTAINER)

mongo-stop: ## Stop the local MongoDB container
	docker stop $(MONGO_CONTAINER)

clean: ## Remove build artifacts
	cargo clean
