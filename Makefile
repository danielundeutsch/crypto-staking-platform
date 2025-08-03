.PHONY: help build up down logs ps test clean

help:
	@echo "Available commands:"
	@echo "  make build    - Build all Docker images"
	@echo "  make up       - Start all services"
	@echo "  make down     - Stop all services"
	@echo "  make logs     - View logs from all services"
	@echo "  make ps       - List running services"
	@echo "  make test     - Run all tests"
	@echo "  make clean    - Clean up volumes and images"

build:
	@echo "Building all services..."
	docker-compose build

up:
	@echo "Starting all services..."
	docker-compose up -d
	@echo "Services are starting up..."
	@echo "Consul UI: http://localhost:8500"
	@echo "RabbitMQ Management: http://localhost:15672 (admin/admin123)"
	@echo "API Gateway: http://localhost:8080"
	@echo "Frontend: http://localhost:3000"
	@echo "Prometheus: http://localhost:9090"
	@echo "Grafana: http://localhost:3001 (admin/admin123)"

down:
	@echo "Stopping all services..."
	docker-compose down

logs:
	docker-compose logs -f

ps:
	docker-compose ps

test:
	@echo "Running tests..."
	@echo "Testing API Gateway..."
	cd services/api-gateway && cargo test
	@echo "Testing Node Template..."
	cd services/node-template && cargo test
	@echo "Testing Frontend..."
	cd frontend && npm test

clean:
	@echo "Cleaning up..."
	docker-compose down -v
	docker system prune -f

# Development commands
dev-api:
	cd services/api-gateway && cargo run

dev-node:
	cd services/node-template && cargo run

dev-frontend:
	cd frontend && npm run dev

# Individual service commands
build-api:
	docker-compose build api-gateway

build-frontend:
	docker-compose build frontend

restart-api:
	docker-compose restart api-gateway

restart-frontend:
	docker-compose restart frontend

# Monitoring commands
metrics:
	@echo "Checking metrics endpoints..."
	@curl -s http://localhost:8080/metrics | head -20
	@echo "\n...truncated"

health:
	@echo "Checking health endpoints..."
	@echo "API Gateway: $$(curl -s http://localhost:8080/health)"
	@echo "Ethereum Node: $$(curl -s http://localhost:8081/health)"
	@echo "Bitcoin Node: $$(curl -s http://localhost:8082/health)"
	@echo "Solana Node: $$(curl -s http://localhost:8083/health)"