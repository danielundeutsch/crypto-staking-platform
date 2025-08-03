# Crypto Staking Platform

A microservices-based platform for staking crypto tokens across multiple blockchain networks.

## Architecture

This platform uses a microservices architecture where each blockchain node connector runs as an independent service.

### Services

- **API Gateway**: Central entry point for all client requests
- **Service Registry**: Dynamic service discovery and health checking
- **Ethereum Node**: Ethereum blockchain connector
- **Bitcoin Node**: Bitcoin blockchain connector  
- **Solana Node**: Solana blockchain connector
- **Staking Orchestrator**: Coordinates staking operations across chains
- **Frontend**: Next.js web application with Tailwind CSS

## Tech Stack

- **Backend**: Rust (Axum, Tokio)
- **Frontend**: Next.js, TypeScript, Tailwind CSS
- **Message Broker**: RabbitMQ
- **Service Discovery**: Consul
- **Orchestration**: Kubernetes
- **CI/CD**: GitHub Actions with SonarQube
- **Monitoring**: Prometheus + Grafana

## Getting Started

### Prerequisites

- Rust 1.75+
- Node.js 20+
- Docker & Docker Compose
- Kubernetes (for production)

### Development Setup

1. Clone the repository
2. Run the development environment:
   ```bash
   docker-compose up -d
   ```

3. Install dependencies:
   ```bash
   # Frontend
   cd frontend && npm install
   
   # Backend services
   cd services/api-gateway && cargo build
   ```

4. Start development servers:
   ```bash
   # Frontend
   cd frontend && npm run dev
   
   # Backend (example for API Gateway)
   cd services/api-gateway && cargo run
   ```

## Project Structure

```
crypto-staking-platform/
├── services/              # Microservices
├── frontend/             # Next.js application
├── shared/               # Shared libraries
├── infrastructure/       # K8s, Docker, Terraform
└── docs/                # Documentation
```

## Development Process

We follow Test-Driven Development (TDD) with 2-week Scrum sprints.

## License

MIT