# Node Service Template

This is a template for creating blockchain node connector services in the crypto staking platform.

## Overview

Each blockchain (Ethereum, Bitcoin, Solana, etc.) will have its own node service based on this template. The service handles:

- Blockchain connectivity
- Staking operations
- Balance queries
- Service registration with Consul
- Health checks
- Prometheus metrics

## Usage

To create a new node service:

1. Copy this template to a new directory (e.g., `ethereum-node`)
2. Update the service name and chain-specific logic
3. Implement the blockchain-specific functionality in `blockchain.rs`
4. Configure environment variables

## Environment Variables

- `SERVICE_NAME`: Name of the service (e.g., "ethereum-node")
- `CHAIN_NAME`: Blockchain name (e.g., "ethereum")
- `HOST`: Service host (default: "0.0.0.0")
- `PORT`: Service port (default: 8080)
- `CONSUL_ADDR`: Consul address (default: "http://localhost:8500")
- `NODE_RPC_URL`: Blockchain node RPC URL
- `LOG_LEVEL`: Logging level (default: "info")

## API Endpoints

- `GET /health`: Health check
- `GET /metrics`: Prometheus metrics
- `POST /stake`: Submit staking request
- `GET /balance/:address`: Query balance
- `GET /info`: Service information

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run service
cargo run
```

## Docker

```bash
# Build image
docker build -t node-template .

# Run container
docker run -p 8080:8080 \
  -e SERVICE_NAME=ethereum-node \
  -e CHAIN_NAME=ethereum \
  -e NODE_RPC_URL=http://localhost:8545 \
  node-template
```